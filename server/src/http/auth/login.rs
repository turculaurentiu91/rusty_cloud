use axum::{http::StatusCode, response::Json, Extension};
use interfaces::{
    auth::{LoginOkResponse, LoginRequest, LoginResponse},
    error_response::ErrorResponse,
};
use sqlx::PgPool;
use tower_cookies::{cookie::time::OffsetDateTime, Cookie, Cookies};

use crate::helpers::user::User;

pub async fn handle(
    Extension(db): Extension<PgPool>,
    cookies: Cookies,
    body: Json<LoginRequest>,
) -> (StatusCode, Json<LoginResponse>) {
    match db::auth::User::login(&body.email, &body.password, &db).await {
        Ok(user) => {
            let token = User::new(user.id, &user.name, &user.email).to_token();
            let days: u64 = chrono::Duration::days(2).num_milliseconds() as u64;
            let days = std::time::Duration::from_millis(days);
            let session_cookie = Cookie::build("session", token)
                .path("/")
                .http_only(true)
                .expires(OffsetDateTime::now_utc() + days)
                .secure(cfg!(debug_assertions))
                .finish();

            cookies.add(session_cookie);
            (
                StatusCode::OK,
                Json(LoginResponse::Success(LoginOkResponse {
                    user: interfaces::auth::User {
                        id: user.id,
                        email: user.email,
                        name: user.name,
                    },
                })),
            )
        }
        Err(e) => match e {
            db::auth::LoginErrors::UserNotFound => (
                StatusCode::UNAUTHORIZED,
                Json(LoginResponse::Error(ErrorResponse {
                    error: "User not found".to_string(),
                    extra: None,
                })),
            ),
            db::auth::LoginErrors::DatabaseError(e) => {
                eprintln!("Error logging in: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(LoginResponse::Error(ErrorResponse {
                        error: "Internal server error".to_string(),
                        extra: None,
                    })),
                )
            }
        },
    }
}
