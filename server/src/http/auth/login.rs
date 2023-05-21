use axum::{http::StatusCode, response::Json, Extension};
use interfaces::auth::{LoginRequest, LoginResponse};
use sqlx::PgPool;

use crate::{helpers::user::User, http::error::Error};

pub async fn handle(
    Extension(db): Extension<PgPool>,
    body: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Error<String>> {
    sqlx::query!(
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
                Ok(Json(LoginResponse {
                    token: user.to_token(),
                }))
            }
            None => Err(Error::new(
                String::from("Unauthorized"),
                StatusCode::UNAUTHORIZED,
            )),
        };
    })
    .map_err(sqlx::Error::into)?
}
