# tgpt

Talk to an AI chatbot without the need for API keys. Available providers: `openai`, `opengpts`, `koboldai`, `phind`, `llama2`, `blackboxai`.

## Usage
- `tgpt "prompt"` : chat with the default provider (GPT-3.5-turbo)
- `tgpt -m` : start multi-line interactive mode
- `tgpt -img "prompt"` : generate images and save them to the current directory
- `tgpt -c "prompt"` : generate code with the default provider (GPT-3.5-turbo)
- `tgpt --provider openai|opengpts|koboldai|phind|llama2|blackboxai -q -w "prompt"` : chat with a specific provider quietly (without animations)
- `tgpt --provider llama2 -s "prompt"` : generate and execute shell commands using a specific provider (with a confirmation prompt)
- `tgpt --provider openai --key "api_key" --model "gpt-3.5-turbo" --max-length 10 --temperature 0.7 --top_p 0.9 "prompt"` : prompt with an API key, model, max response length, temperature, and `top_p` (required when using `openai` provider)
- `tgpt <path/to/file --provider blackboxai "prompt"` : feed a file as additional pre-prompt input

## Categories
shell, files

## Links
- More information: https://github.com/aandrew-me/tgpt
- tldr page: https://tldr.inbrowser.app/pages/common/tgpt
