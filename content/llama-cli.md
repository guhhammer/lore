# llama-cli

Command-line interface for LLM inference.

## Usage
- `llama-cli -m path/to/model` : start up LLM model and interface
- `llama-cli -m path/to/model -c number` : start with setup prompt context size
- `llama-cli -m path/to/model -ngl auto|all|number` : start with set offload number of layer gpu layers
- `llama-cli -m path/to/model -sysf path/to/prompt` : start with sending content as [sys]tem prompt from a file
- `llama-cli -m path/to/model -f path/to/prompt` : start with sending content in file as first prompt

## Categories
files, network, shell

## Links
- More information: https://github.com/ggml-org/llama.cpp#llama-cli
- tldr page: https://tldr.inbrowser.app/pages/linux/llama-cli
