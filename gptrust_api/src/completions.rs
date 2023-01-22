use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Prompt {
    Sentence(String),
    Paragraph(Vec<String>),
    Tokens(u64),
    TokenList(Vec<u64>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RequestStopSequence {
    Sentence(String),
    Paragraph(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCompletionRequest {
    model: String,
    prompt: Prompt,
    suffix: Option<String>,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    n: u32,
    stream: bool,
    logprobs: Option<u32>,
    stop: Option<RequestStopSequence>,
    presence_penalty: f32,
    frequency_penalty: f32,
    best_of: u32,
    logit_bias: Option<HashMap<String, i32>>,
    user: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionLogprobs {
    tokens: Vec<String>,
    token_logprobs: Vec<f32>,
    top_logprobs: Vec<String>,
    text_offset: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionChoice {
    pub text: String,
    index: u64,
    logprobs: Option<CompletionLogprobs>,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<CompletionChoice>,
    usage: CompletionUsage,
}

pub async fn gptrust_complete(
    prompt: String,
) -> Result<Vec<CompletionChoice>, Box<dyn std::error::Error>> {
    let request = CreateCompletionRequest {
        model: String::from("text-ada-001"),
        prompt: Prompt::Sentence(prompt),
        suffix: None,
        max_tokens: 10,
        temperature: 1.0,
        top_p: 1.0,
        n: 1,
        stream: false,
        logprobs: None,
        stop: None, // Some(RequestStopSequence::Sentence(String::from("\n"))),
        presence_penalty: 0.0,
        frequency_penalty: 0.0,
        best_of: 1,
        logit_bias: Some(HashMap::new()),
        user: "gptrust".to_string(),
    };
    let request_body = serde_json::to_string(&request).unwrap();
    // println!("{:#?}", request_body);
    let response_body =
        gptrust_http::openai_http::openai_post("completions".to_string(), request_body).await?;
    // println!("{:#?}", response_body);
    let completion_response: CreateCompletionResponse = serde_json::from_str(&response_body)?;
    Ok(completion_response.choices)
}
