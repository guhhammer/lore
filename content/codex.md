# codex

Natural language code assistant for the terminal, powered by OpenAI. Reads and edits files in your current directory to fulfill requests.

## Usage
- `codex` : start an interactive Codex session in the current directory
- `codex "prompt"` : run a single Codex command using a prompt
- `codex --sandbox workspace-write "prompt"` : run a prompt allowing Codex to edit files in the workspace
- `codex -m model_name "prompt"` : use a specific model
- `codex --oss --local-provider lmstudio|ollama "prompt"` : use a local open source model provider
- `/status` : [Interactive] Display session configuration and token usage
- `codex -h` : display help

## Categories
shell, files

## Links
- More information: https://developers.openai.com/codex/cli/reference
- tldr page: https://tldr.inbrowser.app/pages/common/codex
