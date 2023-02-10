use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    id: String,
    object: String,
    bytes: u64,
    created_at: u64,
    filename: String,
    purpose: String,
}

#[derive(Serialize, Deserialize)]
struct FileResponse {
    object: String,
    data: Vec<File>,
}

pub async fn gptrust_getfiles() -> Result<Vec<File>, Box<dyn std::error::Error>> {
    let body = gptrust_http::openai_get("files".to_string()).await?;
    let file_resp: FileResponse = serde_json::from_str(&body)?;
    Ok(file_resp.data)
}
