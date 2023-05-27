use axum::{
    routing::{get, get_service},
    Extension, Router,
};
use sqlx::PgPool;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod auth;
pub mod error;
mod files;
mod folders;

pub fn app(db: PgPool) -> Router {
    let api = Router::new()
        .route("/", get(me_handler))
        .merge(auth::router())
        .merge(files::router())
        .merge(folders::router())
        .layer(Extension(db));

    let static_files_service =
        get_service(ServeDir::new("../browser/dist").append_index_html_on_directories(true));

    Router::new()
        .fallback(static_files_service)
        .nest("/api", api)
        .layer(CookieManagerLayer::new())
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
