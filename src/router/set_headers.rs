use axum::{headers::ContentType, TypedHeader};

/// Update this route handler so that it returns a content-type header of 'application/json' with the static str of "hello world"
pub async fn set_header() -> (TypedHeader<ContentType>, &'static str) {
    (TypedHeader(ContentType::json()), "hello world")
}
