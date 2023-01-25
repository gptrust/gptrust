mod cli;

#[tokio::main]
async fn main() {
    let response = cli::process_cli().await;
    println!("Result: {:?} ...etc({})", response[0], response.len());
}
