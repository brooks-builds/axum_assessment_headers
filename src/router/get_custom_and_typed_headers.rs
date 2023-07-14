use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

/// Update this route handler to get the following headers from the request and return them in the JSON object JsonHeaders
///
/// - content-type
/// - token
pub async fn get_custom_and_typed_headers() -> Result<Json<JsonHeaders>, StatusCode> {
    todo!()
}

#[derive(Serialize, Deserialize)]
pub struct JsonHeaders {
    content_type: String,
    token: String,
}
