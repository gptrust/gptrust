use rand::seq::SliceRandom;

static CHOICES: &'static [&'static str] = &[
    "airplane",
    "balloon",
    "cat",
    "dog",
    "engine",
    "fish",
    "galaxy",
    "helicopter",
];
const STYLE: &str = "cartoon sketch";

use std::io::Write;
use std::process::Command;

pub fn dump2file(
    file_name: String,
    _directory: &Option<String>,
    body_bytes: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(file_name.clone()).unwrap();
    file.write_all(body_bytes)
        .expect("Could not write data to file");
    Ok(())
}

pub fn dump2screen(
    file_name: String,
    _directory: &Option<String>,
    _body_bytes: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("./imgcat").arg(file_name).status().unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();
    let word = CHOICES.choose(&mut rng);
    let prompt = format!("{:?},{}", word, STYLE);
    let images = gptrust_api::images::generations(prompt.to_string())
        .await
        .expect("Couldn't generate image");
    let names = images
        .iter()
        .map(|x| x.url.clone())
        .collect::<Vec<String>>();
    let img_file =
        gptrust_http::http::save_url(names[0].clone(), None, vec![dump2file, dump2screen])
            .await
            .expect("Can't save the image locally");
    let _rm = std::fs::remove_file(img_file);
}
