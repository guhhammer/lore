# vllm

Serve large language models efficiently and interact with them through an OpenAI-compatible API.

## Usage
- `vllm serve model_name` : launch an OpenAI-compatible API server for a model
- `vllm serve model_name --host 0.0.0.0 --port 8000` : serve a model on a specific host and port
- `vllm chat` : start an interactive chat session
- `vllm run-batch -i input.jsonl -o output.jsonl --model model_name` : run batch prompts from a file and write results to a file
- `vllm collect-env` : print environment information for bug reports
- `vllm --help` : list all available subcommands

## Categories
network

## Links
- More information: https://docs.vllm.ai/en/latest/getting_started/quickstart.html
- tldr page: https://tldr.inbrowser.app/pages/common/vllm
