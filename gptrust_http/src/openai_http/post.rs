use hyper;
use hyper_tls;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};

const OPENAI_BASE_URL: &str = "https://api.openai.com/v1/";
const OPENAI_ORG: &str = "org-ioVS0wAWUCPVBK4x45pqIGCj";
const OPENAI_ENV: &str = "OPENAI_API_KEY";

fn add_field(boundary: String, field: String, value: String) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    write!(data, "--{}\r\n", boundary)?;
    write!(
        data,
        "Content-Disposition: form-data; name=\"{}\"\r\n",
        field
    )?;
    write!(data, "Content-type: text/plain;charset=UTF-8\r\n")?;
    write!(data, "\r\n")?;
    write!(data, "{}", value)?;
    write!(data, "\r\n")?;
    Ok(data)
}

fn add_boundary(boundary: String) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    write!(data, "--{}--\r\n", boundary)?;
    Ok(data)
}
fn generate_boundary() -> String {
    String::from("--AaB03x")
}

fn add_file(boundary: String, filename: String) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    write!(data, "--{}\r\n", boundary)?;
    write!(
        data,
        "Content-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\n",
        filename
    )?;
    write!(data, "Content-Type: application/octet-stream\r\n")?;
    write!(data, "\r\n")?;
    let mut f = File::open(filename)?;
    f.read_to_end(&mut data)?;
    write!(data, "\r\n")?;
    Ok(data)
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

pub async fn openai_post_form(
    subpath: String,
    fields: Vec<String>,
    values: Vec<String>,
    filename: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let full_path = format!("{}{}", OPENAI_BASE_URL, subpath);
    let https = hyper_tls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    let api_key = env::var(OPENAI_ENV).expect("$OPENAI_API_KEY is not set");
    let boundary = generate_boundary();
    let mut data = add_field(boundary.clone(), fields[0].clone(), values[0].clone())?;
    if let Some(x) = filename {
        data.extend(add_file(boundary.clone(), x)?);
    }
    data.extend(add_boundary(boundary.clone())?);

    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri(full_path)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Organization", OPENAI_ORG)
        .header(
            "Content-type",
            format!("multipart/form-data; boundary={}", boundary),
        )
        .body(data.into())?;

    // Pass our request builder object to our client.
    let resp = client.request(req).await?;
    println!("{:#?}", resp);
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
