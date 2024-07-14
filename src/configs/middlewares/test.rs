use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use hyper::header::{COOKIE, SET_COOKIE};
use time::Duration;
use tracing::info;

// layer test 용
pub async fn test_log_and_modify(jar: CookieJar, req: Request, next: Next) -> (CookieJar, Response) {
    info!("Received request");
    let mut response = next.run(req).await;
    let cookie = Cookie::build(("name", "value"))
    .http_only(true)
    .max_age(Duration::seconds(30));
    let jar = jar.add(cookie);
    
    response.headers_mut().insert("x-custom-header", "custom-value".parse().unwrap());
    info!("Modified response");
    
    (jar, response)
}

pub async fn test_cookie_modi(mut req: Request, next: Next) -> Response {
    tracing::info!("test_cookie_modi");
    let acc_token_cookie = Cookie::build(("test_modi", "hahasssssss"))
        .path("/");
    // jar.add(acc_token_cookie);
    tracing::info!("cookie: {}", acc_token_cookie); //acc_token_cookie.to_string().parse().unwrap()
    req.headers_mut().insert(COOKIE, acc_token_cookie.to_string().parse().unwrap());
    let response = next.run(req).await;
    response
}