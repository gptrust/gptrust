use serde::{Deserialize, Serialize};

const AUDIOBASE: &str = "audio";

#[derive(Serialize, Deserialize, Debug)]
pub struct Transcript {
    pub text: String,
}

pub async fn transcriptions(filename: String) -> Result<Transcript, Box<dyn std::error::Error>> {
    let transpath = AUDIOBASE.to_owned() + "/transcriptions";
    match gptrust_http::openai_http::openai_post_form(
        transpath.to_string(),
        vec!["model".to_string()],
        vec!["whisper-1".to_string()],
        Some(filename.clone()),
    )
    .await
    {
        Ok(response_body) => {
            // println!("{:#?}", response_body);
            let upload_response: Transcript = serde_json::from_str(&response_body)?;
            Ok(upload_response)
        }
        Err(e) => Err(e),
    }
}
