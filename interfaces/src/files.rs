use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct File {
    pub id: i32,
    pub file_name: String,
    pub mime_type: String,
    pub size_in_bytes: i64,
    pub formatted_size: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FileIndexResponse {
    pub files: Vec<File>,
}
