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
    "ice cream",
    "japan",
    "kite",
    "lightning",
    "moon",
    "notebook",
    "onion",
    "piano",
    "queue",
    "rowing",
    "snowboard",
    "taj mahal",
    "united states",
    "volcano",
    "warship",
    "x-ray",
    "yankee",
    "zebra",
];
const STYLE: &str = "cartoon sketch";

use std::{
    env,
    fs::File,
    io,
    io::{prelude::*, BufReader, Write},
    path::Path,
    process::Command,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

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
    let args: Vec<String> = env::args().collect();
    let mut input_string = String::new();
    let mut rng = rand::thread_rng();
    let word;
    if args.len() > 1 {
        let filename = &args[1];
        let lines = lines_from_file(filename);
        word = lines.choose(&mut rng).unwrap().clone();
    } else {
        word = CHOICES.choose(&mut rng).unwrap().to_string();
    }
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

    input_string.clear();
    print!("Your guess: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_string).unwrap();
    if word == input_string.trim().to_string() {
        println!("Correct!")
    } else {
        println!("Sorry! It was {}", word)
    }
}
