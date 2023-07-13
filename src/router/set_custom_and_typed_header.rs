use axum::{
    headers::{ContentType, HeaderMapExt},
    http::{HeaderMap, HeaderValue},
};

/// Update this route handler to respond with "content-type: application/json" and "token: Bearer 1234567890" headers together with the static string "hello world"
pub async fn set_custom_and_typed_header() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();

    headers.typed_insert(ContentType::json());

    headers.insert("token", HeaderValue::from_static("Bearer 1234567890"));

    (headers, "hello world")
}
