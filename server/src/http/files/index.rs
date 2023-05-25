use axum::{Extension, Json};
use interfaces::files::{File, FileIndexResponse};
use sqlx::PgPool;

use crate::{helpers::user::User, http::error::Error};

pub async fn handle(
    Extension(db): Extension<PgPool>,
    user: User,
) -> Result<Json<FileIndexResponse>, Error<String>> {
    let files = sqlx::query!(
        r#"
            SELECT
                *,
                pg_size_pretty(f.size_in_bytes) AS formatted_size
            FROM files AS f
            WHERE f.owner_id = $1
        "#,
        user.id
    )
    .fetch_all(&db)
    .await
    .map_err(sqlx::Error::into)?;

    let files: Vec<File> = files
        .iter()
        .map(|db_file| File {
            id: db_file.id,
            file_name: db_file.file_name.to_owned(),
            mime_type: db_file.mime_type.to_owned(),
            formatted_size: db_file.formatted_size.to_owned(),
            size_in_bytes: db_file.size_in_bytes,
        })
        .collect();

    Ok(Json(FileIndexResponse { files }))
}
