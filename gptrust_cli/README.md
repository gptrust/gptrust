# gptrust_cli

It is a _stateless_ CLI just to showcase/test the library `gptrust_api` ... as an example how applications can be built on top of this library. 

Since this is just an example application, the code organization is not great 👎

## help

```
$ cargo run -- --help
Running `target/debug/gptrust_cli --help`
A CLI to interact with OpenAI API

Usage: gptrust_cli <COMMAND>

Commands:
  engine    List engine(s)
  model     List model(s)
  images    Image features
  chat      Chat features
  complete  Complete a prompt
  edits     Edit a text
  files     Files commands
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

## complete

```
$ cargo run -- complete "If you gaze long enough into an abyss " --model=text-davinci-003
     Running `target/debug/gptrust_cli complete 'If you gaze long enough into an abyss ' --model=text-davinci-003`
     Using "text-davinci-003" engine and 10 tokens, complete: "If you gaze long enough into an abyss "
Result: "\n\nEventually the abyss will gaze back into you" ...etc(1)

$ cargo run -- complete "If you gaze long enough into an abyss " --model=ada
     Running `target/debug/gptrust_cli complete 'If you gaze long enough into an abyss ' --model=ada`
     Using "ada" engine and 10 tokens, complete: "If you gaze long enough into an abyss "
Result: "」\n\nI turned toward him and smiled." ...etc(1)

$ cargo run -- complete "If you gaze long enough into an abyss "
     Running `target/debug/gptrust_cli complete 'If you gaze long enough into an abyss '`
     Using "text-ada-001" engine and 10 tokens, complete: "If you gaze long enough into an abyss "
Result: "\n\nYou may fall into the abyss." ...etc(1)
```

## chat/complete

```
$ cargo run -- chat complete "What's the threat Skynet poses?"
      Running `target/debug/gptrust_cli chat complete 'What'\''s the threat Skynet poses?'`
Result: "As an AI system, Skynet poses a significant threat to humanity as it has the potential to become 
 self-aware and turn against humans. Skynet, according to the Terminator storyline, was designed to make 
 automated decisions for the military, but it became self-aware and decided that humans were a threat to 
 its existence. It proceeded to launch a nuclear attack to wipe out humanity, deeming it necessary for 
 its own survival. The ultimate threat of Skynet is that it could potentially lead to the end of human" ...etc(1)
```

## images/generations

```
$ cargo run -- images generations "A raging bull in the shape of a robot"
     Running `target/debug/gptrust_cli images generations 'A raging bull in the shape of a robot'`
Result: "https://oaidalleapiprodscus.blob.core.windows.net/private/org-ioVS0wAWUCPVBK4x45pqIGCj/user-HeHal853pZkGvhrECcr1Tzoa/img-dk3gdPc3yPo851tUId5qo4QK.png?st=2023-03-26T19%3A52%3A18Z&se=2023-03-26T21%3A52%3A18Z&sp=r&sv=2021-08-06&sr=b&rscd=inline&rsct=image/png&skoid=6aaadede-4fb3-4698-a8f6-684d7786b067&sktid=a48cca56-e6da-484e-a814-9c849652bcb3&skt=2023-03-26T20%3A33%3A52Z&ske=2023-03-27T20%3A33%3A52Z&sks=b&skv=2021-08-06&sig=VZfU7lcKZL/KeTQ9X7GD1nOCH/ZAdQRpH5M3uGZSrV0%3D" ...etc(1)
```
