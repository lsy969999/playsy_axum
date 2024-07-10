use axum::{extract::Request, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, Validation};
use crate::{configs::{consts::{ACCESS_TOKEN, REFRESH_TOKEN}, models::auth::Claims}, utils};

/**
 *  access_token 검증
 *      success: 이동
 *      fail: refresh_token 존재 체크
 *          success:db에서 리프레시토큰검증
 *          success: access_token, refresh_token 재발급
 *          fail: 인증에러
 * 
 * access_token
 * 
 * refresh_token
 */
pub async fn auth_cookie_middleware(
    jar: CookieJar,
    req: Request,
    next: Next
) -> Response {
    // access_token_cookie 획득
    let acc_cookie = match jar.get(ACCESS_TOKEN) {
        Some(cookie) => {
            cookie
        }
        // access token이 존재하지 않으면 에러를 발생시킨다.
        None => {
            // return auth error
            todo!();
        }
    };
    let acc = utils::settings::get_settings_jwt_access_keys();
    // access decode 해보기
    let acc_decode = decode::<Claims>(acc_cookie.value(), &acc.decoding, &Validation::default());

    match acc_decode {
        Ok(_) => {
            let response = next.run(req).await;
            response
        }
        Err(error) => {
            // decode 실패하면 계속 진행
            // refreshtoken 체크
            let refr_cookie = match jar.get(REFRESH_TOKEN) {
                Some(cookie) => {
                    cookie
                }
                // refresh token이 존재하지 않으면 에러를 발생시킨다.
                None => {
                    // return auth error
                    todo!();
                }
            };

            let refr = utils::settings::get_settings_jwt_refresh_keys();
            let refr_decode = decode::<Claims>(refr_cookie.value(), &refr.decoding, &Validation::default());
            
            match refr_decode {
                Ok(_) => {
                    // 리프레시 토큰 검증 완료하면
                    // 서버 갔다와서 발급된 토큰인지 조회
                    let is_real = true;

                    if is_real {
                        let mut response = next.run(req).await;
                        response
                    } else {
                        // 서버에서 못찾으면 에러처리
                        todo!();
                    }
                }
                Err(error) => {
                    // 리프레시 토큰 검증실패 ex 만료
                    todo!();
                }
            }
        }
    }
}

pub async fn auth_api_middleware(
    _req: Request,
    _next: Next
) -> Response {
    todo!()
}