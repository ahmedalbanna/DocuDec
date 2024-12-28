use axum::Router;

mod login;
mod signup;

pub fn routes() -> Router {
    Router::new()
        .nest("/login", login::route())
        .nest("/signup", signup::route())
}
