# llm

Interact with Large Language Models (LLMs) via remote APIs and models that can be installed and run on your machine.

## Usage
- `llm keys set openai` : set up an OpenAI API Key
- `llm "Ten fun names for a pet pelican"` : run a prompt
- `cat path/to/file.py | llm -s "Explain this code"` : run a system prompt against a file
- `llm install package1 package2 ...` : install packages from PyPI into the same environment as LLM
- `llm -m orca-mini-3b-gguf2-q4_0 "What is the capital of France?"` : download and run a prompt against a model
- `llm -s 'You are a sentient cheesecake' --save sentient_cheesecake` : create a system prompt and save it with a template name
- `llm chat -m chatgpt -t sentient_cheesecake` : have an interactive chat with a specific model using a specific template

## Categories
shell, files, network

## Links
- More information: https://llm.datasette.io/en/stable/help.html
- tldr page: https://tldr.inbrowser.app/pages/common/llm
