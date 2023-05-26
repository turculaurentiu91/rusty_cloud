use crate::{helpers::user::User, http::error::Error};
use axum::{http::StatusCode, Extension, Json};
use interfaces::files::{CreateOrUpdateFolderRequest, GetFolderResponse};

pub async fn handle(
    Extension(db): Extension<sqlx::PgPool>,
    user: User,
    body: Json<CreateOrUpdateFolderRequest>,
) -> Result<Json<GetFolderResponse>, Error<String>> {
    // test if folder with same name exists
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

    sqlx::query!(
        r#"
            INSERT INTO folders (owner_id, parent_id, name)
            VALUES ($1, $2, $3)
        "#,
        user.id,
        body.parent_id,
        body.name
    )
    .execute(&db)
    .await
    .map_err(sqlx::Error::into)?;

    Ok(Json(GetFolderResponse {
        folders: vec![],
        files: vec![],
    }))
}
