use hyper;
use hyper_tls;
use std::env;

const OPENAI_BASE_URL: &str = "https://api.openai.com/v1/";
const OPENAI_ORG: &str = "org-ioVS0wAWUCPVBK4x45pqIGCj";
const OPENAI_ENV: &str = "OPENAI_API_KEY";

pub async fn openai_get(subpath: String) -> Result<String, Box<dyn std::error::Error>> {
    let full_path = format!("{}{}", OPENAI_BASE_URL, subpath);
    let https = hyper_tls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    let api_key = env::var(OPENAI_ENV).expect("$OPENAI_API_KEY is not set");
    let req = hyper::Request::builder()
        .method(hyper::Method::GET)
        .uri(full_path)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Organization", OPENAI_ORG)
        .body(hyper::Body::from(""))?;

    // Pass our request builder object to our client.
    let resp = client.request(req).await?;
    // println!("{:#?}", resp);
    match resp.status().is_success() {
        true => {
            // Get the response body bytes.
            let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
            // Convert the body bytes to utf-8
            let body: String = String::from_utf8(body_bytes.to_vec()).unwrap();
            Ok(body)
        }
        false => Err(From::from(format!(
            "Request failed, reason: {}",
            resp.status()
        ))),
    }
}

pub async fn openai_post(
    subpath: String,
    body: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let full_path = format!("{}{}", OPENAI_BASE_URL, subpath);
    let https = hyper_tls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    let api_key = env::var(OPENAI_ENV).expect("$OPENAI_API_KEY is not set");
    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri(full_path)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Organization", OPENAI_ORG)
        .header("Content-type", "application/json")
        .body(hyper::Body::from(body))?;

    // Pass our request builder object to our client.
    let resp = client.request(req).await?;
    // println!("{:#?}", resp);
    match resp.status().is_success() {
        true => {
            // Get the response body bytes.
            let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
            // Convert the body bytes to utf-8
            let body: String = String::from_utf8(body_bytes.to_vec()).unwrap();
            Ok(body)
        }
        false => Err(From::from(format!(
            "Request failed, reason: {}",
            resp.status()
        ))),
    }
}
