use std::{fs, io::Write};

use axum::{extract::Multipart, http::StatusCode, Extension};
use sqlx::PgPool;

use crate::{helpers::user::User, http::error::Error};

pub async fn handle(
    Extension(db): Extension<PgPool>,
    user: User,
    mut body: Multipart,
) -> Result<StatusCode, Error<String>> {
    while let Some(field) = body
        .next_field()
        .await
        .map_err(|e| Error::new(e.to_string(), StatusCode::UNPROCESSABLE_ENTITY))?
    {
        let file_name = field.name().unwrap_or_default().to_string();
        let mime_type = field.content_type().unwrap_or_default().to_string();
        let data = field.bytes().await.unwrap_or_default();

        if file_name.is_empty() || mime_type.is_empty() || data.is_empty() {
            continue;
        }

        // Check if the user has a file with this name
        let existing_file = sqlx::query!(
            r#"
                SELECT
                    *
                FROM files AS f
                WHERE f.owner_id = $1
                AND f.file_name = $2
            "#,
            user.id,
            file_name
        )
        .fetch_optional(&db)
        .await
        .map_err(sqlx::Error::into)?;

        let file_id = if let Some(existing_file) = existing_file {
            // File already exists, update metadata in the database
            sqlx::query!(
                r#"
                    UPDATE files
                    SET 
                        mime_type = $1, 
                        size_in_bytes = $2
                    WHERE id = $3
                "#,
                mime_type,
                data.len() as i64,
                existing_file.id,
            )
            .execute(&db)
            .await
            .map_err(sqlx::Error::into)?;
            existing_file.id
        } else {
            // File doesn't exist, insert new file metadata into the database
            let mut transaction = db.begin().await.map_err(sqlx::Error::into)?;

            let file = sqlx::query!(
                r#"
                    INSERT INTO files (owner_id, file_name, mime_type, size_in_bytes, storage_path)
                    VALUES ($1, $2, $3, $4, 'temp')
                    RETURNING *
                "#,
                user.id,
                file_name,
                mime_type,
                data.len() as i64,
            )
            .fetch_one(&mut transaction)
            .await
            .map_err(sqlx::Error::into)?;

            sqlx::query!(
                r#"
                    UPDATE files
                    SET storage_path = '/storage/' || $1
                    WHERE files.id = $1
                "#,
                file.id
            )
            .execute(&mut transaction)
            .await
            .map_err(sqlx::Error::into)?;

            transaction.commit().await.map_err(sqlx::Error::into)?;

            file.id
        };

        let storage_path = format!("/storage/{file_id}");

        // Create the directory if it does not exist
        if let Some(parent_dir) = std::path::Path::new(&storage_path).parent() {
            fs::create_dir_all(parent_dir)
                .map_err(|e| Error::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
        }

        // Open a file handle for writing
        let mut file = fs::File::create(&storage_path)
            .map_err(|e| Error::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

        // Write the file bytes to disk
        file.write_all(&data)
            .map_err(|e| Error::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

        // Close the file handle
        file.flush()
            .map_err(|e| Error::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    }
    Ok(StatusCode::NO_CONTENT)
}
