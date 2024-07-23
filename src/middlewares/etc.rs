use std::fs::metadata;

use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};


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