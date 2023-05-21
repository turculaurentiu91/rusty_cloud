use axum::{routing::post, Router};

mod login;

pub fn router() -> Router {
    Router::new().route("/auth/login", post(login::handle))
}
