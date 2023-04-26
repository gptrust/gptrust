use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateImageRequest {
    prompt: String,
    n: u32,
    size: String,
    response_format: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageData {
    pub url: String,
    b64_json: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ImagesResponse {
    created: u64,
    data: Vec<ImageData>,
}

const IMAGEBASE: &'static str = "images";

pub async fn generations(prompt: String) -> Result<Vec<ImageData>, Box<dyn std::error::Error>> {
    let genpath = IMAGEBASE.to_owned() + "/generations";
    let request = CreateImageRequest {
        prompt,
        n: 1,
        size: "512x512".to_string(),
        response_format: "url".to_string(),
    };
    let request_body = serde_json::to_string(&request).unwrap();
    // println!("{:#?}", request_body);
    match gptrust_http::openai_http::openai_post(genpath.to_string(), request_body).await {
        Ok(response_body) => {
            // println!("{:#?}", response_body);
            let image_resp: ImagesResponse = serde_json::from_str(&response_body)?;
            Ok(image_resp.data)
        }
        Err(e) => Err(e),
    }
}
