use clap::{arg, Command};

fn cli() -> Command {
    Command::new("gptrust_cli")
        .about("A CLI to interact with OpenAI API")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(
            Command::new("engine")
                .about("List engine(s)")
                .arg(arg!([ENGINE] "The name of the engine"))
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("model")
                .about("List model(s)")
                .arg(arg!([MODEL] "The name of the model"))
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("complete")
                .about("Complete a prompt")
                .arg(arg!(<PROMPT> "The text to complete"))
                .arg(
                    arg!(--model <MODEL> "The model to use for completion")
                        .num_args(0..=1)
                        .require_equals(true)
                        .default_value("text-ada-001")
                        .default_missing_value("text-ada-001"),
                )
                .arg(
                    arg!(--"max-tokens" <TOKENS>)
                        .num_args(0..=1)
                        .require_equals(true)
                        .default_value("10")
                        .value_parser(clap::value_parser!(u32).range(3..100)),
                ),
        )
        .subcommand(
            Command::new("edits")
                .about("Edit a text")
                .arg(arg!(<INPUT> "The text to correct"))
                .arg(arg!(<INSTR> "Instruction to fix")),
        )
}

pub async fn process_cli() -> Vec<String> {
    let matches = cli().get_matches();
    let names;
    match matches.subcommand() {
        Some(("engine", sub_matches)) => {
            let specific = sub_matches.get_one::<String>("ENGINE");
            println!("Engine {:?}", specific);
            match specific {
                None => {
                    let engines = gptrust_api::engines::list()
                        .await
                        .expect("Failed to get engines");
                    names = engines
                        .iter()
                        .map(|x| x.id.clone())
                        .collect::<Vec<String>>();
                }
                Some(name) => {
                    let engine = gptrust_api::engines::retrieve(name.clone())
                        .await
                        .expect(format!("Failed to get model {}", name).as_str());
                    let enginename = engine.id.clone();
                    names = vec![enginename];
                }
            }
        }
        Some(("model", sub_matches)) => {
            let specific = sub_matches.get_one::<String>("MODEL");
            println!("Model {:?}", specific);
            match specific {
                None => {
                    let models = gptrust_api::models::list()
                        .await
                        .expect("Failed to get models");
                    names = models.iter().map(|x| x.id.clone()).collect::<Vec<String>>();
                }
                Some(name) => {
                    let model = gptrust_api::models::retrieve(name.clone())
                        .await
                        .expect(format!("Failed to get model {}", name).as_str());
                    let modelname = model.id.clone();
                    names = vec![modelname];
                }
            }
        }
        Some(("complete", sub_matches)) => {
            let engine = sub_matches
                .get_one::<String>("model")
                .map(|s| s.as_str())
                .expect("defaulted in clap");
            let max_tokens = sub_matches.get_one::<u32>("max-tokens").unwrap();
            let prompt = sub_matches
                .get_one::<String>("PROMPT")
                .expect("A prompt is required");
            println!(
                "Using {:?} engine and {:?} tokens, complete: {:?}",
                engine, max_tokens, prompt
            );
            let completions = gptrust_api::completions::complete(
                prompt.to_string(),
                Some(engine.to_string()),
                Some(*max_tokens),
            )
            .await
            .expect("Couldn't complete the prompt");
            names = completions
                .iter()
                .map(|x| x.text.clone())
                .collect::<Vec<String>>();
        }
        Some(("edits", sub_matches)) => {
            let input = sub_matches
                .get_one::<String>("INPUT")
                .expect("Input (text to be edited) is required");
            let instruction = sub_matches
                .get_one::<String>("INSTR")
                .expect("Instructions (how to fix) is required");
            println!("Apply \"{}\" on: \"{}\"", instruction, input);
            let edits =
                gptrust_api::edits::gptrust_edits(input.to_string(), instruction.to_string())
                    .await
                    .expect("Could not get the edits");
            names = edits
                .iter()
                .map(|x| x.text.clone())
                .collect::<Vec<String>>();
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
    names
}
