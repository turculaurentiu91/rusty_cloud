use axum::{http::StatusCode, response::Json, Extension};
use interfaces::{
    auth::{LoginOkResponse, LoginRequest, LoginResponse},
    error_response::ErrorResponse,
};
use sqlx::PgPool;

use crate::helpers::user::User;

pub async fn handle(
    Extension(db): Extension<PgPool>,
    body: Json<LoginRequest>,
) -> (StatusCode, Json<LoginResponse>) {
    match db::auth::User::login(&body.email, &body.password, &db).await {
        Ok(user) => {
            let token = User::new(user.id, &user.name, &user.email).to_token();

            (
                StatusCode::OK,
                Json(LoginResponse::Success(LoginOkResponse { token })),
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
