use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionRequestMessage {
    role: String,
    content: String,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RequestStopSequence {
    Sentence(String),
    Paragraph(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionResponseMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionChoice {
    index: u64,
    message: ChatCompletionResponseMessage,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChatCompletionRequest {
    model: String,
    messages: Vec<ChatCompletionRequestMessage>,
    temperature: f32,
    top_p: f32,
    n: u32,
    stream: bool,
    stop: Option<RequestStopSequence>,
    max_tokens: u32,
    presence_penalty: f32,
    frequency_penalty: f32,
    logit_bias: Option<HashMap<String, i32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChatCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<ChatCompletionChoice>,
    usage: CompletionUsage,
}

pub async fn complete(
    roles: Vec<String>,
    messages: Vec<String>,
    model: Option<String>,
    max_tokens: Option<u32>,
) -> Result<Vec<ChatCompletionChoice>, Box<dyn std::error::Error>> {
    //let request = CreateChatCompletionRequest {
    //model: model.unwrap_or(String::from("gpt-3.5-turbo")),
    //}
}
