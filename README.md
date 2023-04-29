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
sudo apt-get install cargo libssl-dev pkg-config 
```
#### Take a spin:
```
 git clone https://github.com/gptrust/gptrust.git
 cd gptrust/
 OPENAI_API_KEY="sk-pasteyouropeaiapikeyhere" cargo run -- --help
```

### Usage example
Setup the API key:
```
$ export OPENAI_API_KEY=sk-pasteyourapikeyhere 
```
`chat complete` (usually the `system` is ignored, so just using the `user` prompt) (seems gpt knows more than the [wikipedia page](https://en.wikipedia.org/wiki/Oberea_lutea) though I wonder which model _knows_ more
```
$ cargo run -- chat complete --model=gpt-4 --max-tokens=80 "Oberea lutea is a species of"
Result: "beetle in the family Cerambycidae. It is commonly known as the longhorn beetle or the longicorn beetle. 
        These beetles are characterized by their elongated bodies and long antennae, which can be as long as or 
        even longer than their bodies. Oberea lutea is typically yellowish or greenish and can be found in 
        various habitats such as forests, parks" ...etc(1)
$ cargo run -- chat complete --model=gpt-3.5-turbo --max-tokens=80 "Oberea lutea is a species of"
Result: "longhorn beetle found in Europe and western Asia. It is known for its distinctive bright yellow coloration 
         and long antennae, which can be up to twice the length of the beetle's body. The larvae of O. lutea feed on 
         various species of deciduous trees, including oak, beech, and birch. This beetle is not considered a major 
         pest, but large populations can cause" ...etc(1)
```

`images generations` (prints the URL as well downloads it in local directory):
```
$ cargo run -- images generations "A fat cat sitting on top of a TV, cartoon"
Result: "https://oaidalleapiprodscus.blob.core.windows.net/private/org-ioVS0wAWUCPVBK4x45pqIGCj/user-HeHal853pZkGvhrECcr1Tzoa/img-U4nhGTI4zLuviyDQyUJvstue.png?......" ...etc(1)
$ ls -l img*
-rw-rw-r-- 1 ubuntu ubuntu 787387 Apr 29 11:40 img-U4nhGTI4zLuviyDQyUJvstue.png
```
