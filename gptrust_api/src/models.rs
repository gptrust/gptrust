use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Permission {
    id: String,
    object: String,
    created: u64,
    allow_create_engine: bool,
    allow_sampling: bool,
    allow_logprobs: bool,
    allow_search_indices: bool,
    allow_view: bool,
    allow_fine_tuning: bool,
    organization: String,
    group: Option<String>,
    is_blocking: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub id: String,
    object: String,
    created: u64,
    owned_by: String,
    permission: Vec<Permission>,
    root: String,
    parent: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ModelResponse {
    object: String,
    data: Vec<Model>,
}

const MODELBASE: &'static str = "models";

pub async fn list() -> Result<Vec<Model>, Box<dyn std::error::Error>> {
    match gptrust_http::openai_http::openai_get(MODELBASE.to_string()).await {
        Ok(body) => {
            let model_resp: ModelResponse = serde_json::from_str(&body)?;
            Ok(model_resp.data)
        }
        Err(e) => Err(e),
    }
}

pub async fn retrieve(name: String) -> Result<Model, Box<dyn std::error::Error>> {
    let modelpath = MODELBASE.to_owned() + "/" + &name;
    match gptrust_http::openai_http::openai_get(modelpath.to_string()).await {
        Ok(body) => {
            let model_resp: Model = serde_json::from_str(&body)?;
            Ok(model_resp)
        }
        Err(e) => Err(e),
    }
}
