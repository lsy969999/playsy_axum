use std::fs::metadata;

use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use hyper::HeaderMap;


/// compression 의 경우 content-length를 모르는데
/// 오리지널 size를 응답 헤더에 X-Original-Content-Length 로 추가해준다.
pub async fn add_original_content_length(
    req: Request,
    next: Next,
) -> Response {
    let path = format!(".{}", req.uri().path());

    // Get the file metadata
    let response = next.run(req).await;
    // response
    if let Ok(metadata) = metadata(&path) {
        // tracing::debug!("add_original_content_length OK {}", path);
        let size = metadata.len().to_string();
        let mut response = response;
        response.headers_mut().insert("X-Original-Content-Length", HeaderValue::from_str(&size).unwrap());
        response
    } else {
        // tracing::debug!("add_original_content_length NO OK {}", path);
        response
    }
}

pub async fn htmx_hx_header_pass(
    req: Request,
    next: Next,
) -> Response {
    let hx_headers = req.headers()
        .iter()
        .filter_map(|(k, v)| {
            if k.as_str().starts_with("x-hx-") {
                Some((k.clone(), v.clone()))
            } else {
                None
            }
        })
        .collect::<HeaderMap>();
    let mut response = next.run(req).await;
    hx_headers.iter().for_each(|(k,v)|{
        response.headers_mut().insert(k, v.clone());
    });
    response
}