use axum::{
    body::StreamBody,
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Extension,
};
use sqlx::PgPool;
use tokio_util::io::ReaderStream;

use crate::{helpers::user::User, http::error::Error};

pub async fn handle(
    Path(file_id): Path<u32>,
    user: User,
    Extension(db): Extension<PgPool>,
) -> Result<impl IntoResponse, Error<String>> {
    let file = sqlx::query!(
        r#"
            SELECT
                *
            FROM files
            WHERE owner_id = $1
            AND id = $2
            LIMIT 1
        "#,
        user.id,
        file_id as i32
    )
    .fetch_optional(&db)
    .await
    .map_err(sqlx::Error::into)?;

    if let Some(file_metadata) = file {
        let file = tokio::fs::File::open(file_metadata.storage_path)
            .await
            .map_err(|e| Error::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

        let stream = ReaderStream::new(file);
        let body = StreamBody::new(stream);

        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            file_metadata.file_name.parse().unwrap(),
        );

        headers.insert(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file_metadata.file_name)
                .parse()
                .unwrap(),
        );

        return Ok((headers, body));
    }

    Err(Error::new(String::from("not found"), StatusCode::NOT_FOUND))
}
