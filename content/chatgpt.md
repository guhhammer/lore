# chatgpt

Shell script to use OpenAI's ChatGPT and DALL·E from the terminal.

## Usage
- `chatgpt` : start in chat mode
- `chatgpt -p "What is the regex to match an email address?"` : give a prompt to answer to
- `chatgpt -m gpt-4` : start in chat mode using a specific model (default is `gpt-3.5-turbo`)
- `chatgpt -i "You are Rick, from Rick and Morty. Respond to questions using his mannerism and include insulting jokes."` : start in chat mode with an initial prompt
- `echo "How to view running processes on Ubuntu?" | chatgpt` : pipe the result of a command to `chatgpt` as a prompt
- `chatgpt -p "image: A white cat"` : generate an image using DALL·E

## Categories
shell, development

## Links
- More information: https://github.com/0xacx/chatGPT-shell-cli
- tldr page: https://tldr.inbrowser.app/pages/common/chatgpt
