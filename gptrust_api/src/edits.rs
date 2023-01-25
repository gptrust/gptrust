use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEditRequest {
    model: String,
    input: String,
    instruction: String,
    n: u32,
    temperature: f32,
    top_p: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditsLogprobs {
    tokens: Vec<String>,
    token_logprobs: Vec<f32>,
    top_logprobs: Vec<String>,
    text_offset: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditsChoice {
    pub text: String,
    index: u64,
    logprobs: Option<EditsLogprobs>,
    finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditsUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEditResponse {
    id: Option<String>,
    object: String,
    created: u64,
    model: Option<String>,
    choices: Vec<EditsChoice>,
    usage: EditsUsage,
}

pub async fn gptrust_edits(
    input: String,
    instruction: String,
) -> Result<Vec<EditsChoice>, Box<dyn std::error::Error>> {
    let request = CreateEditRequest {
        model: String::from("text-davinci-edit-001"),
        input: input,
        instruction: instruction,
        n: 1,
        temperature: 1.0,
        top_p: 1.0,
    };
    let request_body = serde_json::to_string(&request).unwrap();
    // println!("{:#?}", request_body);
    let response_body =
        gptrust_http::openai_http::openai_post("edits".to_string(), request_body).await?;
    // println!("{:#?}", response_body);
    let edit_response: CreateEditResponse = serde_json::from_str(&response_body)?;
    Ok(edit_response.choices)
}
