use axum::{routing::post, Router};

mod login;
mod register;

pub fn router() -> Router {
    Router::new()
        .route("/auth/login", post(login::handle))
        .route("/auth/register", post(register::handle))
}
