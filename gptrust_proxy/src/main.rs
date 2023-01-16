#[tokio::main]

async fn main() {
    println!("Printing the list model names: ");
    let models = gptrust_api::models::gptrust_getmodels()
        .await
        .expect("Failed to get models");
    let names = models.iter().map(|x| x.id.clone()).collect::<Vec<String>>();
    println!("{} {:#?}", names.len(), names);
}
