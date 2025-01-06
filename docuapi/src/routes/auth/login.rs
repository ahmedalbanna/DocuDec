use actix_web::Responder;
use tracing::instrument;

#[instrument]
#[actix_web::post("/login")]
pub async fn login() -> impl Responder {
    "hello"
}
