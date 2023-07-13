mod set_custom_and_typed_header;
mod set_custom_header;
mod set_header;

use axum::routing::get;
use axum::Router;
use set_custom_and_typed_header::set_custom_and_typed_header;
use set_custom_header::set_custom_header;
use set_header::set_header;

pub fn create_router() -> Router {
    Router::new()
        .route("/set_header", get(set_header))
        .route("/set_custom_header", get(set_custom_header))
        .route(
            "/set_custom_and_typed_header",
            get(set_custom_and_typed_header),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
