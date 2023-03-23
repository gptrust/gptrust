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
            println!("{:#?}", response_body);
            let upload_response: OpenAIFile = serde_json::from_str(&response_body)?;
            Ok(upload_response)
        }
        Err(e) => Err(e),
    }
}
