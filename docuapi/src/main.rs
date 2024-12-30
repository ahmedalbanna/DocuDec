use axum::Router;
use tracing::info;

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod routes;

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter) // Add filtering based on RUST_LOG
        .with(fmt::layer().with_line_number(true).with_thread_names(true))
        .init();

    info!("Starting the application...");

    let routes = Router::new()
        .nest("/auth", routes::auth::routes().await)
        .nest("/gen", routes::gen::routes().await)
        .nest("/file", routes::file::route().await);

    let tcp = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Port 3000 is not avaliabel");

    match tcp.local_addr() {
        Ok(addr) => tracing::info!("Started server on: {}", addr),
        Err(error) => {
            tracing::error!("Failed to look up the tcp address for logging, error: {error}")
        }
    }

    if let Err(error) = axum::serve(tcp, routes.into_make_service()).await {
        tracing::error!("Failed to serve the API error: {error}")
    }
}
