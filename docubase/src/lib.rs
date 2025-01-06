pub mod auth;
mod error;
pub mod models;
pub mod utils;

pub async fn initialize_db() {
    let port = std::env::var("DATABASE_PORT")
        .expect("Failed to get the DATABASE_PORT env, make sure to set it")
        .parse::<u16>()
        .expect("Failed to parse the DATABASE_PORT as a number");
}
