use clap::{arg, Command};
use std::io::Read;

use crate::utils::dump2file;

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
            Command::new("images")
                .about("Image features")
                .subcommand_required(true)
                .subcommand(
                    Command::new("generations")
                        .about("Generate an image")
                        .arg(arg!(<PROMPT> "Prompt to generate the image")),
                ),
        )
        .subcommand(
            Command::new("chat")
                .about("Chat features")
                .subcommand_required(true)
                .subcommand(
                    Command::new("complete")
                        .about("Complete an ongoing chat")
                        .arg(arg!(<PROMPT> "Prompt said by the user"))
                        .arg(
                            arg!(--model <MODEL> "The model to use for completion")
                                .num_args(0..=1)
                                .require_equals(true)
                                .default_value("gpt-3.5-turbo")
                                .default_missing_value("gpt-3.5-turbo"),
                        )
                        .arg(
                            arg!(--"max-tokens" <TOKENS>)
                                .num_args(0..=1)
                                .require_equals(true)
                                .default_value("256")
                                .value_parser(clap::value_parser!(u32).range(3..4097)),
                        ),
                ),
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
                        .default_value("128")
                        .value_parser(clap::value_parser!(u32).range(3..4097)),
                ),
        )
        .subcommand(
            Command::new("edits")
                .about("Edit a text")
                .arg(arg!(<INPUT> "The text to correct"))
                .arg(arg!(<INSTR> "Instruction to fix")),
        )
        .subcommand(
            Command::new("files")
                .about("Files commands")
                .subcommand_required(true)
                .subcommand(Command::new("list").about("List uploaded files"))
                .subcommand(
                    Command::new("upload")
                        .about("Upload a file for fine tuning")
                        .arg(arg!(<FILENAME> "Name of the jsonl file")),
                ),
        )
        .subcommand(
            Command::new("audio")
                .about("Audio commands")
                .subcommand_required(true)
                .subcommand(
                    Command::new("transcriptions")
                        .about("Upload an audio file for transcription")
                        .arg(arg!(<FILENAME> "Name of the MP3 file")),
                ),
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
                        .unwrap_or_else(|_| panic!("Failed to get model {}", name));
                    let enginename = engine.id;
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
                        .unwrap_or_else(|_| panic!("Failed to get model {}", name));
                    let modelname = model.id;
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
        Some(("chat", sub_matches)) => match sub_matches.subcommand() {
            Some(("complete", more_matches)) => {
                let engine = more_matches
                    .get_one::<String>("model")
                    .map(|s| s.as_str())
                    .expect("defaulted in clap");
                let max_tokens = more_matches.get_one::<u32>("max-tokens").unwrap();
                let prompt = more_matches
                    .get_one::<String>("PROMPT")
                    .expect("A prompt is required");
                let prompt_str;
                if prompt.starts_with('@') {
                    let promptfile = prompt.get(1..);
                    let reader = match promptfile {
                        Some(filename) => {
                            String::from_utf8(std::fs::read(filename).expect("Can't open file"))
                                .expect("Error reading")
                        }
                        None => String::from(""),
                    };
                    prompt_str = reader;
                } else if prompt == "-" {
                    let mut input = String::new();
                    std::io::stdin()
                        .read_to_string(&mut input)
                        .expect("Error reading stdin");
                    prompt_str = input.clone();
                } else {
                    prompt_str = prompt.clone();
                }
                let images = gptrust_api::chat::complete(
                    vec!["user".to_string()],
                    vec![prompt_str.to_string()],
                    Some(engine.to_string()),
                    Some(*max_tokens),
                )
                .await
                .expect("Couldn't complete the prompt");
                names = images
                    .iter()
                    .map(|x| x.message.content.clone())
                    .collect::<Vec<String>>();
            }
            _ => unreachable!(),
        },

        Some(("images", sub_matches)) => match sub_matches.subcommand() {
            Some(("generations", more_matches)) => {
                let prompt = more_matches
                    .get_one::<String>("PROMPT")
                    .expect("A prompt is required");
                let images = gptrust_api::images::generations(prompt.to_string())
                    .await
                    .expect("Couldn't complete the prompt");
                names = images
                    .iter()
                    .map(|x| x.url.clone())
                    .collect::<Vec<String>>();
                let img_file =
                    gptrust_http::http::save_url(names[0].clone(), None, vec![dump2file])
                        .await
                        .expect("Can't save the image locally");
                println!("./imgcat {}", img_file);
            }
            _ => unreachable!(),
        },
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
        Some(("audio", sub_matches)) => match sub_matches.subcommand() {
            Some(("transcriptions", more_matches)) => {
                let prompt = more_matches
                    .get_one::<String>("FILENAME")
                    .expect("A filename is required");
                let transcript = gptrust_api::audio::transcriptions(prompt.to_string())
                    .await
                    .expect("Couldn't transcribe the audio");
                names = vec![transcript.text];
            }
            _ => unreachable!(),
        },
        Some(("files", sub_matches)) => match sub_matches.subcommand() {
            Some(("upload", more_matches)) => {
                let prompt = more_matches
                    .get_one::<String>("FILENAME")
                    .expect("A filename is required");
                let fileuploaded = gptrust_api::files::upload(prompt.to_string())
                    .await
                    .expect("Couldn't upload the file");
                names = vec![fileuploaded.id];
            }
            Some(("list", _more_matches)) => {
                let filelist = gptrust_api::files::list()
                    .await
                    .expect("Can't list our files");
                names = filelist
                    .iter()
                    .map(|x| x.id.clone())
                    .collect::<Vec<String>>();
            }
            _ => unreachable!(),
        },

        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
    names
}
