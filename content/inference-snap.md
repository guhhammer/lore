# inference-snap

Chat with generative AI Language Models that runs locally without internet. Here inference-snap is one of the supported models: `deepseek-r1`, `gemma3`, `gemma4`, `nemotron3-nano`, `qwen-vl`.

## Usage
- `inference-snap chat` : start a chat in terminal (and a chat server in background on first start)
- `inference-snap status` : show current engine and status of chat server
- `sudo inference-snap use-engine engine` : select specific engine (eg. `cuda` for NVIDIA GPU). Download and use the appropriate model for the engine
- `sudo inference-snap show-machine` : show detailed device hardware information (RAM, architecture, etc.)
- `inference-snap get` : get all configuration (`http.host`, `http.port`) of running chat server

## Categories
network, editors

## Links
- More information: https://documentation.ubuntu.com/inference-snaps/
- tldr page: https://tldr.inbrowser.app/pages/linux/inference-snap
