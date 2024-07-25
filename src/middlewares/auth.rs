
use axum::{extract::Request, http::HeaderValue, middleware::Next, response::{IntoResponse, Response}};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, Validation};
use sqlx::PgConnection;
use base64::prelude::*;
use crate::{configs::consts::{ACCESS_TOKEN, REFRESH_TOKEN, USER_INFO}, extractors::database_connection::DatabaseConnection, models::{claims::{AccessClaims, RefreshClaims}, fn_args::token::GenAccessTokenArgs}, repositories::{self}, utils};

/// 이 미들웨어는 액세스 토큰 쿠키로부터 사용자 정보를 읽어와서
/// 리퀘스트 헤더에 user_info 키값으로 사용자 정보를 넣어주어 로그인을 유지시켜주는 미들웨어이다.
/// 
/// 2개의 쿠키를 체크한다. ACCESS_TOKEN, REFRESH_TOKEN 이다.
/// 
/// process_access_token 함수를 통해 ACCESS_TOKEN 유효 여부를 체크한다.
/// 이게 성공한다면 요청헤더에 사용자 정보가 담기고 나머지 과정은 스킵된다.
/// 
/// 만약 ACCESS_TOKEN 검증이 실패한다면 재발급 로직이 실행된다.
/// process_refresh_token 함수를 통해 REFRESH_TOKEN 유효 여부를 체크한다.
/// 이게 성공한다면 요청헤더게 사용자 정보가 담기고 액세스 토큰이 재발급된다.
/// 
/// NOTE1:  
/// HeaderValue에는 ASCII 값만 입력될수 있다.
/// 만약 ASCII값이 아닌 데이터가 HeaderValue에 들어가게되면
/// 이후 값을 뽑는 과정에서 Err가 발생할수 있다.
/// 따라서 토큰에서 뽑아낸 정보를 바로 HeaderValue에 인서트하지 않고
/// Base64 인코딩을 한뒤 인서트 해준다.
/// 사용한는곳에는 당연히 디코딩해서 쓴다.
/// 
/// NOTE2:  
/// 사용자 정보를 액세스토큰에서 읽는 방식이라
/// 매번 디비를 조회하는것보다 성능상 이점이 있지만,
/// 사용자 정보를 항상 최신화 하여 가지고 있게 하기는 힘들다.
/// 따라서 만약 최신화 해야하는게 있다면 액세스토큰을 지워주도록한다.
/// 그러면 다음 접속때 재발급되면서 최신화가 될것이다.
pub async fn set_user_info_from_cookie_to_header(
    DatabaseConnection(mut conn): DatabaseConnection,
    mut jar: CookieJar,
    mut req: Request,
    next: Next
) -> Response {
    let is_acc_chk_success = match process_access_token(&jar, &mut req) {
        Ok(v) => v,
        Err(err) => {
            tracing::error!("process_access_token err: {:?}", err);
            false
        },
    };

    match process_refresh_token(&mut conn, &mut req, &jar, is_acc_chk_success).await {
        // 요청 헤더에 사용자정보를 세팅하고, 재발급된 액세스토큰을 응답 헤더에 세팅해준다.
        Ok(Some(reissued_token)) => {
            let cookie = utils::cookie::generate_access_token_cookie(reissued_token);
            jar = jar.add(cookie);
        }
        // 만약 이전 액세스토큰 검증과정에서도 사용자 정보를 세팅하지 못했고
        // 이쪽에 도달한다면 이후 next 진행에는 사용자 정보가 담지기 못한다.
        Ok(None) => {}
        // 리프레시 토큰 검증과정에서 에러가 발생한다면
        // 보안을 위해 리프레시 토큰을 지워 준다.
        Err(err) => {
            tracing::error!("process refresh token err {err} remove refres token");
            let ref_token_cookie = utils::cookie::generate_refresh_token_remove_cookie();
            jar = jar.remove(ref_token_cookie);
        }
    };

    let response = next.run(req).await;
    (jar, response).into_response()
}

/// 액세스 토큰 체크
/// 
/// case1: return false  
/// 액세스 토큰이 존재하지 않거나 액세스 토큰이 존재하지만 토큰 검증에 실패한다면
/// false를 반환하여 리프레시토큰을 통한 액세스토큰 재발급을 유도한다.
/// 
/// case2: return true  
/// 액세스 토큰이 존재하고 액세스토큰 검증이 완료되었다면
/// 액세스 토큰으로부터 사용자 정보를 읽고
/// 요청 헤더에 user_info 키값으로 사용자 정보를 인서트한다.
/// true를 반환하여 이후 리프레시 토큰 프로세스 스킵을 유도 한다.
fn process_access_token(
    jar: &CookieJar,
    req: &mut Request,
) -> anyhow::Result<bool> {
    Ok(
        match jar.get(ACCESS_TOKEN) {
            Some(acc_tk) => {
                // get acc key
                let acc_keys = utils::config::get_config_jwt_access_keys();
                // acc token validate
                let acc_decode_result = decode::<AccessClaims>(acc_tk.value(), &acc_keys.decoding, &Validation::default());
                let acc_decode = match acc_decode_result {
                    Ok(val) => val,
                    Err(err) => {
                        tracing::warn!("access_token_validate fail err: {:?}", err);
                        return Ok(false)
                    }
                };
                // claim struct -> json
                let json_str = serde_json::to_string(&acc_decode.claims)?;
                // json -> base64
                let user_info_str = BASE64_STANDARD.encode(json_str.as_bytes());
                let hv = user_info_str.parse::<HeaderValue>()?;
                // header setting
                req.headers_mut().insert(USER_INFO, hv);
                true
            }
            None => false
        }
    )
}


/// 리프레시 토큰 체크
/// 
/// case1:  
/// 리프레시 토큰이 없는 경우 아무것도 하지 않고 None을 반환한다.
/// 
/// case2:  
/// 리프레시 토큰이 있는데, 이전 액세스토큰 검증에 성공했다면 None을 반환한다.
/// 이케이스는 이전 액세스토큰 검증하면서 요청헤더에 사용자 정보가 담기게 된다.
/// 
/// case3:
/// 리프레시 토큰이 있는데, 이전 액세스토큰 검증에 실패하여 재발급 시도를 해야하는경우이다.
/// 
/// case3-1:
/// 리프레시 토큰 검증을 실패하면 에러를 발생시켜 리프레시 토큰을 지워준다.
/// 
/// case3-2:
/// 리프레시 토큰 검증이 성공하고, 디비체크를 하는데 정보가 없다면,
/// 에러를 발생시켜서 토큰을 지워준다.
/// 
/// case3-3:
/// 리프레시 토큰 검증도 통과하고, 디비체크도 통과했다면
/// 액세스토큰을 재발급하여 요청헤더에 사용자 정보를 넣어주고
/// 재발급된 토큰을 반환하여 응답헤더에 액세스토큰을 넣어준다.
/// 
/// TODO:  
/// 소셜 로그인 사용자의경우 자체 액세스토큰 재발급 과정에서
/// 프로바이더의 액세스토큰 검증및 재발급을 진행해야된다.
/// 만약 성공한다면 재발급받은 프로바이더 액세스토큰을 디비에 세팅해준다.
/// 만약 실패한다면 이 소셜로그인 사용자는 리프레시토큰을 지워준다.
/// 그러기위해 각 프로바이더별 액세스토큰 리프레시하는 로직을 작성해주고
/// 여기서 진행해야된다.
/// (근데 개인적으로, 프로바이더별로 사용하는 api가 없다면
/// 굳이 최신 프로바이더 액세스토큰을 유지해야될까? 생각이든다.
/// 아직 사용하는곳이 없으니 후순위로두고 TODO로 남긴다.)
async fn process_refresh_token(
    conn: &mut PgConnection,
    req: &mut Request,
    jar: &CookieJar,
    is_acc_chk_success: bool,
) -> anyhow::Result<Option<String>> {
    Ok(
        match jar.get(REFRESH_TOKEN) {
            // refresh token 존재
            Some(refr_tk)=> {
                // 만약 액세스 토큰 검증이 완료됐다면
                // 리프레시 토큰 검증과정은 생략하도록 한다.
                if is_acc_chk_success {
                    return Ok(None)
                }

                // get refr key
                let refr_keys = utils::config::get_config_jwt_refresh_keys();
                // refr token validate
                let refr_decode_result = decode::<RefreshClaims>(refr_tk.value(), &refr_keys.decoding, &Validation::default());
                let refr_decode = match refr_decode_result {
                    Ok(v) => v,
                    Err(err) => {
                        tracing::warn!("refresh token validate fail err: {:?}", err);
                        anyhow::bail!("refreh token validate fail")
                    }
                };
                // refr token db check
                let db_chk = repositories::refresh_token::select_refresh_token_user_by_sn(conn, refr_decode.claims.chk as i32).await?;
                match db_chk {
                    // 리프레시토큰 db 체크도 성공했으니, 액세스 클레임을 만들어서 재발급 시켜준다.
                    Some(rtu) => {
                        // gen acc claim
                        let access_claims = utils::jwt::generate_accesss_claim(GenAccessTokenArgs {
                            avatar_url: rtu.avatar_url,
                            nick_name: rtu.nick_name,
                            user_sn: rtu.user_sn.to_string(),
                        });

                        // claim strcut -> json
                        let json_str = serde_json::to_string(&access_claims)?;
                        // json -> base64
                        let user_info_str = BASE64_STANDARD.encode(json_str.as_bytes());
                        let hv = user_info_str.parse::<HeaderValue>()?;
                        // header setting
                        req.headers_mut().insert(USER_INFO, hv);

                        // claim struct -> token
                        let acc_keys = utils::config::get_config_jwt_access_keys();
                        let reissued_token = utils::jwt::generate_jwt(&access_claims, &acc_keys.encoding)?;
                        Some(reissued_token)
                    }
                    // 디비 체크 실패
                    // 강제로 에러를 발생시켜, 사용자가 가진 리프레시 토큰을 제거해준다.
                    None => anyhow::bail!("refreh token db check fail")
                }
            }
            // refresh token 미존재 그냥 진행
            None => None,
        }
    )
}

/// bearer 헤더를 읽고 헤더에 사용자 정보를 제이슨으로 세팅한다.  
/// bearer api 요청이기에 여기서 리프레시를 통한 액세스 재발급은 해주지않는다.  
/// 만약 재발급을 원한다면 재발급 api를 통해 액세스토큰을 재발급받고 이후에 진행 한다.  
pub async fn set_user_info_from_bearer_to_header(
    _req: Request,
    _next: Next
) -> Response {
    todo!()
}