# qwen

Launch an interactive prompt with Qwen3-Coder.

## Usage
- `qwen` : start a REPL session to chat interactively
- `echo "Summarize the history of Rome" | qwen -p` : send the output of another command to Qwen and exit immediately
- `qwen -m model_name` : override the default model (default: qwen3-coder-max)
- `qwen -s` : run inside a sandbox container
- `qwen -i "Give me an example of recursion in Python"` : execute a prompt then stay in interactive mode
- `qwen -a` : include all files in context
- `qwen --show-memory-usage` : show memory usage in status bar

## See also
- gemini

## Categories
shell, monitoring

## Links
- More information: https://github.com/QwenLM/qwen-code
- tldr page: https://tldr.inbrowser.app/pages/common/qwen
