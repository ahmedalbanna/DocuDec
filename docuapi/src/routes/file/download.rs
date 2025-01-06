use actix_web::Responder;
use tracing::instrument;

#[instrument]
#[actix_web::get("/download")]
pub async fn download() -> impl Responder {
    "Hello, World!"
}
