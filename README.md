# gptrust
### Rust bindings to OpenAI, GPT
Unofficial and community maintained library,
MIT Licensed

### Goals
We have two main goals of this project.

1. Create a library to provide a convenient access to openai API and hide all the complexity of raw json over HTTP (and headers), i.e. implement https://github.com/openai/openai-openapi/blob/master/openapi.yaml

2. Using that library, create a proxy so that any other program can use openai (irrespective of the language they are written). They can communicate to the proxy over a pipe. E.g a python application can do a simply (pseudocode) `with open("/dev/chatgpt") as gpt: gpt.write(); gpt.read()` and the whole complexity of auth, billing will be managed by the proxy

3. Create a CLI frontend, which will just take relevant parameters and `post` them to the API. Maybe we can call it `openaictl` (like kubectl)

### Impatient run

#### Pre-requisite

On a vanilla Ubuntu LTS

```
sudo apt-get update
sudo apt-get install cargo libssl-dev pkg-config emacs-nox rustfmt
```

#### Take a spin:

```
 git clone https://github.com/gptrust/gptrust.git
 cd gptrust/
 OPENAI_API_KEY="sk-pasteyouropeaiapikeyhere" cargo run -- --help
```
