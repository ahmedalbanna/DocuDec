use actix_web::web::{self, ServiceConfig};

pub mod download;

pub fn config(config: &mut ServiceConfig) {
    config.service(web::scope("/file").service(download::download));
}
