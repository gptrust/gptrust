use hyper;
use hyper_tls;
use std::io::Write;

pub async fn save_url(
    url_path: String,
    _directory: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = url_path.parse::<hyper::Uri>().unwrap();
    let https = hyper_tls::HttpsConnector::new();

    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    let file_name = String::from(url.path().split('/').last().unwrap());

    let resp = client.get(url).await?;
    match resp.status().is_success() {
        true => {
            let mut file = std::fs::File::create(file_name).unwrap();
            let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
            file.write_all(&body_bytes)
                .expect("Could not write data to file");
            Ok(())
        }
        false => Err(From::from(format!(
            "Request failed, reason: {}",
            resp.status()
        ))),
    }
}
