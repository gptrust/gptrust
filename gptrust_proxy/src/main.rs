#[tokio::main]

async fn main() {
    println!("Hello, world!\nAnd ");
    gptrust_api::gptrust_getmodels()
        .await
        .expect("Failed to get models");
}
