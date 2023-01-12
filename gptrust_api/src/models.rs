use std::env;
use hyper;
use hyper_tls;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Permission {
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
struct Model {
       id: String,
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
       
pub async fn gptrust_getmodels()  ->  Result<(), Box<dyn std::error::Error>> {

    let https = hyper_tls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");
    let req = hyper::Request::builder()
        .method(hyper::Method::GET)
        .uri("https://api.openai.com/v1/models")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Organization", "org-ioVS0wAWUCPVBK4x45pqIGCj")
        .body(hyper::Body::from(""))?;

    // Pass our request builder object to our client.
    let resp = client.request(req).await?;

    // Get the response body bytes.
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;

    // Convert the body bytes to utf-8
    let body = String::from_utf8(body_bytes.to_vec()).unwrap();

    let model_resp: ModelResponse = serde_json::from_str(&body)?;
    println!("{:#?}", model_resp.data[0]);

    Ok(())

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
