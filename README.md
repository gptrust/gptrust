# gptrust
### Rust bindings to OpenAI, GPT

### Goals
We have two main goals of this project.

1. Create a library to provide a convenient access to openai API and hide all the complexity of raw json over HTTP (and headers), i.e. implement https://github.com/openai/openai-openapi/blob/master/openapi.yaml

2. Using that library, create a proxy so that any other program can use openai (irrespective of the language they are written). They can communicate to the proxy over a pipe. E.g a python application can do a simply (pseudocode) `with open("/dev/chatgpt") as gpt: gpt.write(); gpt.read()` and the whole complexity of auth, billing will be managed by the proxy
