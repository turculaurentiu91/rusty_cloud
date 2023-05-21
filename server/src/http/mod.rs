use axum::{routing::get, Extension, Router};
use sqlx::PgPool;

pub mod error;

pub fn app(db: PgPool) -> Router {
    Router::new()
        .route("/", get(me_handler))
        .layer(Extension(db))
}

pub async fn me_handler() -> String {
    String::from("Hello")
}

pub async fn serve(db: PgPool) {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app(db).into_make_service())
        .await
        .unwrap()
}
