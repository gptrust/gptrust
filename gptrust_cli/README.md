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
