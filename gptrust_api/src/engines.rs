use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Engine {
    pub id: String,
    object: String,
    owner: String,
    ready: bool,
}

#[derive(Serialize, Deserialize)]
struct EngineResponse {
    object: String,
    data: Vec<Engine>,
}

pub async fn gptrust_getengines() -> Result<Vec<Engine>, Box<dyn std::error::Error>> {
    let body = gptrust_http::openai_http::openai_get("engines".to_string()).await?;
    let engine_resp: EngineResponse = serde_json::from_str(&body)?;
    Ok(engine_resp.data)
}
