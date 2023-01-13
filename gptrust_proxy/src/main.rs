#[tokio::main]

async fn main() {
    println!("Printing the first model: ");
    gptrust_api::models::gptrust_getmodels()
        .await
        .expect("Failed to get models");
}
