use axum::{extract::Request, middleware::Next, response::Response};
use axum_extra::extract::{cookie::Cookie, CookieJar};
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