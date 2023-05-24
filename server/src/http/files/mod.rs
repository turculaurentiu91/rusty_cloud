use axum::{
    routing::{get, put},
    Router,
};

mod create_or_update;
mod index;

pub fn router() -> Router {
    Router::new()
        .route("/files", get(index::handle))
        .route("/files", put(create_or_update::handle))
}
