use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIFile {
    pub id: String,
    object: String,
    bytes: u64,
    created_at: u64,
    filename: String,
    purpose: String,
    status: String,
    status_details: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListFilesResponse {
    object: String,
    data: Vec<OpenAIFile>,
}

pub async fn upload(filename: String) -> Result<OpenAIFile, Box<dyn std::error::Error>> {
    match gptrust_http::openai_http::openai_post_form(
        "files".to_string(),
        vec!["purpose".to_string()],
        vec!["fine-tune".to_string()],
        Some(filename.clone()),
    )
    .await
    {
        Ok(response_body) => {
            // println!("{:#?}", response_body);
            let upload_response: OpenAIFile = serde_json::from_str(&response_body)?;
            Ok(upload_response)
        }
        Err(e) => Err(e),
    }
}

pub async fn list() -> Result<Vec<OpenAIFile>, Box<dyn std::error::Error>> {
    match gptrust_http::openai_http::openai_get("files".to_string()).await {
        Ok(response_body) => {
	    // println!("{:#?}", response_body);
            let filelist: ListFilesResponse = serde_json::from_str(&response_body)?;
            Ok(filelist.data)
        }
        Err(e) => Err(e),
    }
}
