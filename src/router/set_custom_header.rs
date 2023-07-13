use axum::http::{HeaderMap, HeaderValue};

/// Update this route handler to return a header "token" with value "Bearer 1234567890"
pub async fn set_custom_header() -> HeaderMap {
    let mut outgoing_headers = HeaderMap::new();
    outgoing_headers.insert("token", HeaderValue::from_static("Bearer 1234567890"));

    outgoing_headers
}
