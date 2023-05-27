use axum::{http::StatusCode, Extension, Json};
use interfaces::{
    auth::{RegisterRequest, RegisterResponse, User},
    error_response::ErrorResponse,
};
use sqlx::PgPool;

pub async fn handle(
    Extension(db): Extension<PgPool>,
    body: Json<RegisterRequest>,
) -> (StatusCode, Json<RegisterResponse>) {
    match db::auth::User::register(&body.name, &body.email, &body.password, &db).await {
        Ok(user) => {
            let user = User {
                id: user.id,
                name: user.name,
                email: user.email,
            };

            (StatusCode::OK, Json(RegisterResponse::Success(user)))
        }
        Err(e) => match e {
            db::auth::RegisterErrors::EmailTaken => (
                StatusCode::UNAUTHORIZED,
                Json(RegisterResponse::Error(ErrorResponse {
                    error: "Email taken".to_string(),
                    extra: None,
                })),
            ),
            db::auth::RegisterErrors::DatabaseError(e) => {
                eprintln!("Error registering: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(RegisterResponse::Error(ErrorResponse {
                        error: "Internal server error".to_string(),
                        extra: None,
                    })),
                )
            }
        },
    }
}
