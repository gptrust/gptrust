use hyper;
use hyper_tls;

type Callback = fn(String, Option<String>, &[u8]) -> Result<(), Box<dyn std::error::Error>>;

pub async fn save_url(
    url_path: String,
    _directory: Option<String>,
    callback: Option<Callback>,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = url_path.parse::<hyper::Uri>().unwrap();
    let https = hyper_tls::HttpsConnector::new();

    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    let file_name = String::from(url.path().split('/').last().unwrap());

    let resp = client.get(url).await?;
    match resp.status().is_success() {
        true => {
            let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
            match callback {
                Some(function) => {
                    function(file_name.clone(), _directory, &body_bytes)?;
                }
                None => {}
            }
            Ok(file_name)
        }
        false => Err(From::from(format!(
            "Request failed, reason: {}",
            resp.status()
        ))),
    }
}
