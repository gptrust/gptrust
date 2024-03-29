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

const ENGINEBASE: &str = "engines";

pub async fn list() -> Result<Vec<Engine>, Box<dyn std::error::Error>> {
    match gptrust_http::openai_http::openai_get(ENGINEBASE.to_string()).await {
        Ok(body) => {
            let engine_resp: EngineResponse = serde_json::from_str(&body)?;
            Ok(engine_resp.data)
        }
        Err(e) => Err(e),
    }
}

pub async fn retrieve(name: String) -> Result<Engine, Box<dyn std::error::Error>> {
    let enginepath = ENGINEBASE.to_owned() + "/" + &name;
    match gptrust_http::openai_http::openai_get(enginepath.to_string()).await {
        Ok(body) => {
            let engine_resp: Engine = serde_json::from_str(&body)?;
            Ok(engine_resp)
        }
        Err(e) => Err(e),
    }
}
