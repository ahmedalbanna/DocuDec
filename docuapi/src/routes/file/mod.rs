use crate::mw::my_middleware;
use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig},
};

pub mod download;

pub fn config(config: &mut ServiceConfig) {
    config.service(
        web::scope("/file")
            .wrap(from_fn(my_middleware))
            .service(download::download),
    );
}
