use core::iter::zip;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionRequestMessage {
    role: String,
    content: String,
    // name: Option<String>,
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
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionChoice {
    index: u64,
    pub message: ChatCompletionResponseMessage,
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
    let prompts: Vec<ChatCompletionRequestMessage> = zip(roles, messages)
        .map(|(r, m)| ChatCompletionRequestMessage {
            role: r,
            content: m,
            // name: None,
        })
        .collect();
    let request = CreateChatCompletionRequest {
        model: model.unwrap_or_else(|| String::from("gpt-3.5-turbo")),
        messages: prompts,
        max_tokens: max_tokens.unwrap_or(100),
        temperature: 1.0,
        top_p: 1.0,
        n: 1,
        stream: false,
        stop: None,
        presence_penalty: 0.0,
        frequency_penalty: 0.0,
        logit_bias: Some(HashMap::new()),
    };
    let request_body = serde_json::to_string(&request).unwrap();
    // println!("{:#?}", request_body);
    match gptrust_http::openai_http::openai_post("chat/completions".to_string(), request_body).await
    {
        Ok(response_body) => {
            // println!("{:#?}", response_body);
            let completion_response: CreateChatCompletionResponse =
                serde_json::from_str(&response_body)?;
            Ok(completion_response.choices)
        }
        Err(e) => Err(e),
    }
}
