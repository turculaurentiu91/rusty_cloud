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
    let user = sqlx::query!(
        r#"
            SELECT 
                *
            FROM users
            WHERE email = $1
            AND password = crypt($2, password)
        "#,
        body.email,
        body.password,
    )
    .fetch_optional(&db)
    .await
    .map(|result| {
        return match result {
            Some(result) => {
                let user = User::new(result.id, &result.name, &result.email);
                (
                    StatusCode::OK,
                    Json(LoginResponse::Success(LoginOkResponse {
                        token: user.to_token(),
                    })),
                )
            }
            None => (
                StatusCode::UNAUTHORIZED,
                Json(LoginResponse::Error(ErrorResponse {
                    error: "Invalid email or password".to_string(),
                    extra: None,
                })),
            ),
        };
    });

    match user {
        Ok(user) => user,
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LoginResponse::Error(ErrorResponse {
                error: e.to_string(),
                extra: None,
            })),
        ),
    }
}
