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
                        .default_missing_value("text-ada-001")
                )
                .arg(
                    arg!(--"max-tokens" <TOKENS>)
                        .num_args(0..=1)
                        .require_equals(true)
                        .default_value("10")
                        .value_parser(clap::value_parser!(u32).range(3..100))
                ),
        )

}


fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("engine", sub_matches)) => {
            println!(
                "Engine {:?}",
                sub_matches.get_one::<String>("ENGINE")
            );
        }
        Some(("model", sub_matches)) => {
            println!(
                "Model {:?}",
                sub_matches.get_one::<String>("MODEL")
            );
        }
        Some(("complete", sub_matches)) => {
            let engine = sub_matches
                .get_one::<String>("model")
                .map(|s| s.as_str())
                .expect("defaulted in clap");
            let max_tokens = sub_matches
                .get_one::<u32>("max-tokens").unwrap();
            let prompt = sub_matches.get_one::<String>("PROMPT").expect("A prompt is required");
            println!("Using {:?} engine and {:?} tokens, complete: {:?}", engine, max_tokens, prompt);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    // Continued program logic goes here...
}


