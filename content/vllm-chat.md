# vllm chat

Generate chat completions via the running API server using the OpenAI-compatible REST API.

## Usage
- `vllm chat --quick "message"` : send a single chat prompt to the API server and get a response
- `vllm chat --model-name model_name --quick "message"` : chat with a specific model name
- `vllm chat --system-prompt "system_prompt" --quick "message"` : chat with a custom system prompt
- `vllm chat --url http://custom-host:port/v1 --quick "message"` : chat using a custom API server URL
- `vllm chat --api-key your_api_key --quick "message"` : chat with an API key for authentication
- `vllm chat` : start an interactive chat session

## Categories
network, shell

## Links
- More information: https://docs.vllm.ai/en/latest/serving/openai_compatible_server.html
- tldr page: https://tldr.inbrowser.app/pages/common/vllm-chat
