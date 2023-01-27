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

const ENGINEBASE: &'static str = "engines";

pub async fn list() -> Result<Vec<Engine>, Box<dyn std::error::Error>> {
    let body = gptrust_http::openai_http::openai_get(ENGINEBASE.to_string()).await?;
    let engine_resp: EngineResponse = serde_json::from_str(&body)?;
    Ok(engine_resp.data)
}

pub async fn retrieve(name: String) -> Result<Engine, Box<dyn std::error::Error>> {
    let enginepath = ENGINEBASE.to_owned() + "/" + &name;
    let body = gptrust_http::openai_http::openai_get(enginepath.to_string()).await?;
    let engine_resp: Engine = serde_json::from_str(&body)?;
    Ok(engine_resp)
}
