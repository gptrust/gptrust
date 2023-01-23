use std::env;

#[tokio::main]

async fn main() {
    let args: Vec<String> = env::args().collect();
    let topic = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("science fiction")
    };
    println!("Printing the list of model names: ");
    let models = gptrust_api::models::gptrust_getmodels()
        .await
        .expect("Failed to get models");
    let names = models.iter().map(|x| x.id.clone()).collect::<Vec<String>>();
    println!(" {:?} ...etc({})", &names[1..5], names.len());
    println!("Printing the list engine names: ");
    let engines = gptrust_api::engines::gptrust_getengines()
        .await
        .expect("Failed to get engines");
    let names = engines
        .iter()
        .map(|x| x.id.clone())
        .collect::<Vec<String>>();
    println!(" {:?} ...etc({})", &names[1..5], names.len());
    let prompt = format!("Name a {} movie.", topic);
    println!("Prompt> {}", prompt);
    let movies = gptrust_api::completions::gptrust_complete(prompt)
        .await
        .expect("Couldn't complete the prompt");
    let names = movies
        .iter()
        .map(|x| x.text.clone())
        .collect::<Vec<String>>();
    println!(" {:?} (total {} responses)", names[0], names.len());
}
