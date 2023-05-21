use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use interfaces::error_response::ErrorResponse;
use sqlx::Error as SqlxError;
use validator::ValidationErrors;

pub struct Error<Extra>(StatusCode, Json<ErrorResponse<Extra>>);

impl Error<ValidationErrors> {
    pub fn validation(validation_errors: ValidationErrors) -> Self {
        Self(
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                extra: Some(validation_errors),
                error: String::from("Validation errors"),
            }),
        )
    }
}

impl Error<String> {
    pub fn new(message: String, status_code: StatusCode) -> Self {
        Self(status_code, Json(ErrorResponse::new(message)))
    }

    pub fn internal_server_error(message: String) -> Self {
        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(message)),
        )
    }
}

impl IntoResponse for Error<String> {
    fn into_response(self) -> Response {
        let mut response = self.1.into_response();
        *response.status_mut() = self.0;

        response
    }
}

impl IntoResponse for Error<ValidationErrors> {
    fn into_response(self) -> Response {
        let mut response = self.1.into_response();
        *response.status_mut() = self.0;

        response
    }
}

impl Into<Error<String>> for SqlxError {
    fn into(self) -> Error<String> {
        Error::internal_server_error(self.to_string())
    }
}
