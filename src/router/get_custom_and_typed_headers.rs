use axum::headers::{ContentType, HeaderMapExt};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::{Deserialize, Serialize};

/// Update this route handler to get the following headers from the request and return them in the JSON object JsonHeaders
///
/// - content-type
/// - token
pub async fn get_custom_and_typed_headers(
    headers: HeaderMap,
) -> Result<Json<JsonHeaders>, StatusCode> {
    let Some(content_type ) = headers.typed_get::<ContentType>() else { return Err(StatusCode::BAD_REQUEST)};
    let token = match headers.get("token") {
        Some(token) => token,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    let json_headers = JsonHeaders {
        content_type: content_type.to_string(),
        token: token
            .to_str()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .to_owned(),
    };

    Ok(Json(json_headers))
}

#[derive(Serialize, Deserialize)]
pub struct JsonHeaders {
    content_type: String,
    token: String,
}
