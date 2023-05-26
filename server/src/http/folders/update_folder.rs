use axum::{extract::Path, http::StatusCode, Extension, Json};
use interfaces::files::CreateOrUpdateFolderRequest;

use crate::{helpers::user::User, http::error::Error};

pub async fn handle(
    Path(folder_id): Path<i32>,
    user: User,
    Extension(db): Extension<sqlx::PgPool>,
    body: Json<CreateOrUpdateFolderRequest>,
) -> Result<StatusCode, Error<String>> {
    // check if folder exists and it's the owner
    let folder = sqlx::query!(
        r#"
            SELECT
                *
            FROM folders
            WHERE owner_id = $1
            AND id = $2
        "#,
        user.id,
        folder_id
    )
    .fetch_optional(&db)
    .await
    .map_err(sqlx::Error::into)?;

    if let None = folder {
        return Err(Error::new(
            "Folder does not exist".to_owned(),
            StatusCode::BAD_REQUEST,
        ));
    }

    // check if folder with same name exists
    let folder = sqlx::query!(
        r#"
            SELECT
                *
            FROM folders
            WHERE owner_id = $1
            AND parent_id = $2
            AND name = $3
        "#,
        user.id,
        body.parent_id,
        body.name
    )
    .fetch_optional(&db)
    .await
    .map_err(sqlx::Error::into)?;

    if let Some(_) = folder {
        return Err(Error::new(
            "Folder with same name already exists".to_owned(),
            StatusCode::BAD_REQUEST,
        ));
    }

    // if parent folder is changed, check if it exists and it's the owner
    if let Some(parent_id) = body.parent_id {
        let parent = sqlx::query!(
            r#"
                SELECT
                    *
                FROM folders
                WHERE owner_id = $1
                AND id = $2
            "#,
            user.id,
            parent_id
        )
        .fetch_optional(&db)
        .await
        .map_err(sqlx::Error::into)?;

        if let None = parent {
            return Err(Error::new(
                "Parent folder does not exist".to_owned(),
                StatusCode::BAD_REQUEST,
            ));
        }
    }

    sqlx::query!(
        r#"
            UPDATE folders
            SET
                name = $1,
                parent_id = $2
            WHERE id = $3
        "#,
        body.name,
        body.parent_id,
        folder_id
    )
    .execute(&db)
    .await
    .map_err(sqlx::Error::into)?;

    Ok(StatusCode::OK)
}
