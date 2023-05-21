use axum::{http::StatusCode, Extension, Json};
use interfaces::auth::RegisterRequest;
use sqlx::PgPool;

use crate::http::error::Error;

pub async fn handle(
    Extension(db): Extension<PgPool>,
    body: Json<RegisterRequest>,
) -> Result<StatusCode, Error<String>> {
    let existing_user = sqlx::query!(r#"SELECT id FROM users WHERE email = $1"#, body.email)
        .fetch_optional(&db)
        .await
        .map_err(sqlx::Error::into)?;

    if let Some(_) = existing_user {
        return Err(Error::new(
            format!(
                "User with email {} already exists in our database",
                body.email
            ),
            StatusCode::BAD_REQUEST,
        ));
    }

    sqlx::query!(
        r#"
            INSERT INTO
                users(name, email, password)
            VALUES (
                $1,
                $2,
                crypt($3, gen_salt('bf', 8))
            );
        "#,
        body.name,
        body.email,
        body.password
    )
    .execute(&db)
    .await
    .map_err(sqlx::Error::into)?;

    Ok(StatusCode::NO_CONTENT)
}
