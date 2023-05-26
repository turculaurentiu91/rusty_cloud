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

#[derive(Serialize, Deserialize)]
pub struct Folder {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetFolderResponse {
    pub folders: Vec<Folder>,
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOrUpdateFolderRequest {
    pub name: String,
    pub parent_id: Option<i32>,
}
