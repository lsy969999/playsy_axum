
use axum::{extract::Request, http::HeaderValue, middleware::Next, response::{IntoResponse, Response}};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;
use jsonwebtoken::{decode, Validation};
use sqlx::PgConnection;
use time::{Duration, OffsetDateTime};
use crate::{configs::consts::{ACCESS_TOKEN, REFRESH_TOKEN, USER_INFO}, extractors::database_connection::DatabaseConnection, models::claims::{AccessClaims, RefreshClaims}, repositories::{self}, utils};

/// 이 미들웨어는 헤더에 유저값 세팅만 관여한다. 에러반환은 하지 않는다.
/// case 1:
///     access_token: 존재 && 유효,
///     refresh_token: 있거나 없어도 됨,
///         access_token decode 후 헤더에 넣어주고 헤더 세팅 후 진행
/// case 2:
///     access_token: 미존재 || (존재 && 미유효),
///     refresh_token: 유효, 존재,
///         reresh_token 사용해서 access_token 재발급(with db) 받고 헤더 세팅 후 진행 하며,
///             이때 재발급된 토큰들은 response에 잘 세팅해준다.
///             재발급에 문제가 발생하면 헤더 세팅하지 않고 진행
/// case 3:
///     access_token: 미존재 || (존재 && 미유효)
///     refresh_token: 미존재 || (존재 && 미유효)
///         헤더 세팅하지 않고 진행
///         refresh_token 제거
pub async fn set_user_info_from_cookie_to_header(
    DatabaseConnection(mut conn): DatabaseConnection,
    mut jar: CookieJar,
    mut req: Request,
    next: Next
) -> Response {
    let is_acc_chk_success = match process_access_token(&jar, &mut req) {
        Ok(v) => v,
        Err(_) => false,
    };

    match process_refresh_token(&mut conn, &mut req, &jar, is_acc_chk_success).await {
        // 재발급 토큰이 존재한다면 리스폰스에 세팅해서 액세스토큰쿠키를 세팅해준다.
        Ok(Some(reissued_token)) => {
            let cookie = utils::cookie::generate_access_token_cookie(reissued_token);
            jar = jar.add(cookie);
        }
        Ok(None) => {}
        Err(err) => {
            tracing::error!("process refresh token err {err} remove refres token");
            let ref_token_cookie = utils::cookie::generate_refresh_token_remove_cookie();
            jar = jar.remove(ref_token_cookie);
        }
    };

    // 진행
    let response = next.run(req).await;
    (jar, response).into_response()
}

fn process_access_token(
    jar: &CookieJar,
    req: &mut Request,
) -> anyhow::Result<bool> {
    Ok(
        match jar.get(ACCESS_TOKEN) {
            Some(acc_tk) => {
                let acc_keys = utils::config::get_config_jwt_access_keys();
                let acc_decode = decode::<AccessClaims>(acc_tk.value(), &acc_keys.decoding, &Validation::default())?;
                let str = serde_json::to_string(&acc_decode.claims)?;
                // Some(str)
                let hv: HeaderValue = str.parse()?;
                req.headers_mut().insert(USER_INFO, hv);
                // true
                true
            }
            None => false
        }
    )
}



async fn process_refresh_token(
    conn: &mut PgConnection,
    req: &mut Request,
    jar: &CookieJar,
    is_acc_chk_success: bool,
) -> anyhow::Result<Option<String>> {
    Ok(
        match jar.get(REFRESH_TOKEN) {
            Some(refr_tk) if !is_acc_chk_success => {
                let refr_keys = utils::config::get_config_jwt_refresh_keys();
                let refr_decode = decode::<RefreshClaims>(refr_tk.value(), &refr_keys.decoding, &Validation::default())?;
                let db_chk = repositories::refresh_token::select_refresh_token_user_by_sn(conn, refr_decode.claims.chk as i32).await?;
                match db_chk {
                    Some(rtu) => {
                        let now: OffsetDateTime = OffsetDateTime::now_utc();
                        let acc_keys = utils::config::get_config_jwt_access_keys();
                        let acc_exp = *utils::config::get_config_jwt_access_time();
                        let access_claims = AccessClaims::new(rtu.user_sn.to_string(), now + Duration::seconds(acc_exp), now, None, rtu.nick_name, rtu.avatar_url);
                        
                        let str = serde_json::to_string(&access_claims)?;
                        req.headers_mut().insert(USER_INFO, str.parse()?);
    
                        let reissued_token = utils::jwt::generate_jwt(&access_claims, &acc_keys.encoding)?;
                        Some(reissued_token)
                    }
                    None =>  {
                        Err(anyhow::anyhow!("db_check fail"))?
                    }
                }
            }
            _ => None
        }
    )

    // todo!()
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

/// 만약 헤더에 사용자 정보가 안담겨 있으면 UnAuthorized 리턴을 해버린다.  
/// 이 미들웨어는 반드시 인증정보가 필요한 요청에 사용한다.
/// 미들웨어 사용순서에 주의한다.
pub async fn validate_user_info_from_header(
    // headers: HeaderMap,
    req: Request,
    next: Next
) -> Response {
    if req.headers().get(USER_INFO).is_none() {
        tracing::debug!("[validate_user_info_from_header] USER_INFO none!, return 401");
        return (StatusCode::UNAUTHORIZED).into_response()
    }

    next.run(req).await
}