use std::io::{self, BufRead};

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let engine = String::from("gpt-3.5-turbo");
    let max_tokens = 256;
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,
            Ok(inline) => {
                // println!("> {}", <String as AsRef<str>>::as_ref(&inline)); // Arrrgh so much to print a line
                let resp = gptrust_api::chat::complete(
                    vec!["user".to_string()],
                    vec![inline.to_string()],
                    Some(engine.to_string()),
                    Some(max_tokens),
                )
                .await
                .expect("Couldn't complete the prompt");
                let ans = resp
                    .iter()
                    .map(|x| x.message.content.clone())
                    .collect::<Vec<String>>();
                let mut iterator = ans.iter();
                while let Some(outline) = iterator.next() {
                    println!("{}", outline);
                }
            }
        }
    }
}
