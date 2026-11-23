# openai

CLI tool providing access to the OpenAI API.

## Usage
- `openai api models.list` : list models
- `openai api completions.create --model ada --prompt "Hello world"` : create a completion
- `openai api chat_completions.create --model gpt-3.5-turbo --message user "Hello world"` : create a chat completion
- `openai api image.create --prompt "two dogs playing chess, cartoon" --num-images 1` : generate images via DALL·E API

## Categories
containers, media

## Links
- More information: https://github.com/openai/openai-python
- tldr page: https://tldr.inbrowser.app/pages/common/openai
