mod cli;

#[tokio::main]
async fn main() {
    let response = cli::process_cli().await;
    println!("{}", response[0]);
}
