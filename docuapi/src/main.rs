//! the main entry point to the api
//! we run it from here
//!
//! routes -> contains all api routes
//!
//! mw -> the middlewares for the api
//!
//! utils -> a couple of utilities related the api

pub mod mw;
pub mod routes;
pub mod utils;

use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use docubase::DocuBase;
use routes::{auth, file, gen};
use tracing::info;

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use utils::json_error_handler;

#[actix_web::main]
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

    let database = Arc::new(
        DocuBase::new()
            .await
            .unwrap_or_else(|err| panic!("couldn't connect to database, error: {err}")),
    );

    //let user = database
    //    .create_user(
    //        "abdullah".into(),
    //        "abdu.albanna@proton.me".into(),
    //        "12345".into(),
    //    )
    //    .await
    //    .unwrap();

    //println!("id: {:#?}, created_at: {:#?}", user.id, user.created_at);

    HttpServer::new(move || {
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    // TODO: restrict it
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Restrict HTTP methods
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::AUTHORIZATION,
                    ])
                    .supports_credentials(), // Allow cookies/auth headers if necessary
            )
            .app_data(
                web::JsonConfig::default().error_handler(json_error_handler::json_error_handler),
            )
            .app_data(web::Data::new(Arc::clone(&database)))
            .configure(gen::config)
            .configure(file::config)
            .configure(auth::config)
    })
    .bind((address, port))
    .expect("Failed to start out the server")
    .workers(4)
    .run()
    .await
    .unwrap_or_else(|err| panic!("something wrong with actix HttpServer, error: {err}"));
}
