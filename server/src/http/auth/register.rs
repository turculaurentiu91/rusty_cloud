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
    let existing_user = sqlx::query!(r#"SELECT id FROM users WHERE email = $1"#, body.email)
        .fetch_optional(&db)
        .await;

    match existing_user {
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterResponse::Error(ErrorResponse {
                    error: e.to_string(),
                    extra: None,
                })),
            );
        }
        Ok(Some(_)) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(RegisterResponse::Error(ErrorResponse {
                    error: format!(
                        "User with email {} already exists in our database",
                        body.email
                    ),
                    extra: None,
                })),
            );
        }
        Ok(None) => {
            let result = sqlx::query!(
                r#"
                    INSERT INTO
                        users(name, email, password)
                    VALUES (
                        $1,
                        $2,
                        crypt($3, gen_salt('bf', 8))
                    )
                    RETURNING id, name, email;
                "#,
                body.name,
                body.email,
                body.password
            )
            .fetch_one(&db)
            .await;

            match result {
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(RegisterResponse::Error(ErrorResponse {
                            error: e.to_string(),
                            extra: None,
                        })),
                    );
                }
                Ok(db_user) => (
                    StatusCode::OK,
                    Json(RegisterResponse::Success(User {
                        id: db_user.id,
                        name: db_user.name,
                        email: db_user.email,
                    })),
                ),
            }
        }
    }
}
