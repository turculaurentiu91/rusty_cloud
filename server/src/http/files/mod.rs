use axum::{routing::get, Router};

mod index;

pub fn router() -> Router {
    Router::new().route("/files", get(index::handle))
}
