use std::net::SocketAddr;

use eyre::Result;
use router::create_router;

mod router;

pub async fn run() -> Result<()> {
    tracing_subscriber::fmt::init();
    let router = create_router();
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Server running on port 3000");
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
