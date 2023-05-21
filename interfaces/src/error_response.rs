use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse<Extra> {
    pub error: String,
    pub extra: Option<Extra>,
}
impl ErrorResponse<String> {
    pub fn new(error: String) -> ErrorResponse<String> {
        ErrorResponse { error, extra: None }
    }
}
