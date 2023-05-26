use axum::{
    routing::{get, post},
    Router,
};

mod create_folder;
mod get_folder;

pub fn router() -> Router {
    Router::new()
        .route("/folders/:id", get(get_folder::handle))
        .route("/folders", post(create_folder::handle))
}
