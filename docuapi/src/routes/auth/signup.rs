use actix_web::Responder;
use tracing::instrument;

#[instrument]
#[actix_web::post("/signup")]
pub async fn signup() -> impl Responder {
    "hello"
}
