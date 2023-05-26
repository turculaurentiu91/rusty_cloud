use axum::{routing::get, Router};

mod get_folder;

pub fn router() -> Router {
    Router::new().route("/folders/:id", get(get_folder::handle))
}
