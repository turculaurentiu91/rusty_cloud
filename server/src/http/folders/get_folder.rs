use axum::{extract::Path, Extension, Json};
use interfaces::files::{File, Folder, GetFolderResponse};
use sqlx::PgPool;

use crate::{helpers::user::User, http::error::Error};

pub async fn handle(
    Path(folder_id): Path<Option<i32>>,
    Extension(db): Extension<PgPool>,
    user: User,
) -> Result<Json<GetFolderResponse>, Error<String>> {
    let folders = sqlx::query!(
        r#"
            SELECT
                *
            FROM folders
            WHERE owner_id = $1
            AND parent_id = $2
        "#,
        user.id,
        folder_id
    )
    .fetch_all(&db)
    .await
    .map_err(sqlx::Error::into)?
    .iter()
    .map(|db_folder| Folder {
        id: db_folder.id,
        name: db_folder.name.to_owned(),
        parent_id: db_folder.parent_id,
    })
    .collect();

    let files = sqlx::query!(
        r#"
            SELECT
                *,
                pg_size_pretty(f.size_in_bytes) AS formatted_size
            FROM files AS f
            WHERE f.owner_id = $1
            AND f.folder_id = $2
        "#,
        user.id,
        folder_id
    )
    .fetch_all(&db)
    .await
    .map_err(sqlx::Error::into)?
    .iter()
    .map(|db_file| File {
        id: db_file.id,
        file_name: db_file.file_name.to_owned(),
        mime_type: db_file.mime_type.to_owned(),
        formatted_size: db_file.formatted_size.to_owned(),
        size_in_bytes: db_file.size_in_bytes,
    })
    .collect();

    Ok(Json(GetFolderResponse { folders, files }))
}
