use actix_web::web::{self, ServiceConfig};

mod login;
mod signup;

pub fn config(config: &mut ServiceConfig) {
    config.service(
        web::scope("/auth")
            .service(login::login)
            .service(signup::signup),
    );
}
