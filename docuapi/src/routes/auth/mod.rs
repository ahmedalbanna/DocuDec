use actix_web::web::{self, ServiceConfig};

pub mod error;
mod login;
mod signup;

pub fn config(config: &mut ServiceConfig) {
    config.service(
        web::scope("/auth")
            .service(login::login)
            .service(signup::signup),
    );
}
