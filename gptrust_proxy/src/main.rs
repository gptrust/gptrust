#[tokio::main]

async fn main() {
    println!("Printing the list of model names: ");
    let models = gptrust_api::models::gptrust_getmodels()
        .await
        .expect("Failed to get models");
    let names = models.iter().map(|x| x.id.clone()).collect::<Vec<String>>();
    println!("({}) {:?}", names.len(), names);
    println!("Printing the list engine names: ");
    let engines = gptrust_api::engines::gptrust_getengines()
        .await
        .expect("Failed to get engines");
    let names = engines
        .iter()
        .map(|x| x.id.clone())
        .collect::<Vec<String>>();
    println!("({}) {:?}", names.len(), names);
    let movies =
        gptrust_api::completions::gptrust_complete(String::from("Name a science fiction movie"))
            .await
            .expect("Couldn't complete the prompt");
    let names = movies
        .iter()
        .map(|x| x.text.clone())
        .collect::<Vec<String>>();
    println!("({}) {:?}", names.len(), names);
}
