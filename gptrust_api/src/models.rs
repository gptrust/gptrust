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

pub async fn gptrust_getmodels() -> Result<Vec<Model>, Box<dyn std::error::Error>> {
    let body = gptrust_http::openai_http::openai_get("models".to_string()).await?;
    let model_resp: ModelResponse = serde_json::from_str(&body)?;
    Ok(model_resp.data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
