mod routes;
mod utils;

use actix_web::{web, App, HttpServer};
use routes::{auth, file, gen};
use tracing::info;

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use utils::json_error_handler;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to find a .env file, make sure it's in the root directory");

    let port = *utils::constants::PORT;
    let address = utils::constants::ADDRESS.to_owned();

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter) // Add filtering based on RUST_LOG
        .with(fmt::layer().with_line_number(true).with_thread_names(true))
        .init();

    info!("Starting the application...");

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::JsonConfig::default().error_handler(json_error_handler::json_error_handler),
            )
            .configure(gen::config)
            .configure(file::config)
            .configure(auth::config)
    })
    .bind((address, port))
    .expect("Failed to start out the server")
    .workers(4)
    .run()
    .await
    .unwrap();
}
