# gptrust_api

gptrust_api is a library that can used to interact with OpenAI API endpoints. It is unofficial and community maintained library, MIT Licensed.

Often it simplifies the API call by supplying sensible defaults. However if adjusting all parameters are needed, 
please file an issue or submit a PR with a elaborate version of the call.

Code example:

    let completions = gptrust_api::completions::complete(
                "Once upon a time, ".to_string(),
                "text-davinci-001".to_string(),
                100,
                )
                .await
                .expect("Couldn't complete the prompt");
     let names = completions
                .iter()
                .map(|x| x.text.clone())
                .collect::<Vec<String>>();
