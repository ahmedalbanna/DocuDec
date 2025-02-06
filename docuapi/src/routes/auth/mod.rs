use actix_web::web::{self, ServiceConfig};

pub mod error;
pub mod login;
pub mod signup;

pub use login::login as Login;
pub use signup::signup as Signup;

pub fn config(config: &mut ServiceConfig) {
    config.service(web::scope("/auth").service(Login).service(Signup));
}
