# fm

Interact with Apple Foundation Models, the language models built into macOS.

## Usage
- `sudo fm license` : review and accept the license terms (required once per machine before first use)
- `fm respond "What is Swift?"` : generate a response to a prompt
- `fm chat -i "You are a coding assistant"` : start an interactive chat session with custom instructions
- `cat path/to/file.txt | fm respond -i "Summarize this text in two sentences"` : process text from `stdin` using instructions (`stdin` is ignored if a prompt is also given)
- `fm schema object --name Person --string name --int age >path/to/schema.json` : generate a JSON schema for an object with a string and an integer property
- `fm respond "Amit is 42 years old" --schema path/to/schema.json` : generate a JSON response that conforms to a schema
- `fm count-tokens "Hello world"` : count the tokens in a prompt
- `fm serve --port 8080` : start an OpenAI-compatible Chat Completions API server on a specific port

## Categories
text, files, network, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/fm.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/fm
