# copilot

Interact with GitHub Copilot.

## Usage
- `copilot` : start interactive mode
- `copilot --allow-tool write` : allow all file editing
- `copilot --continue` : resume the most recent session
- `copilot --resume` : resume a previous session using session picker
- `copilot --model "gpt-5"` : use a specific model
- `copilot --allow-tool 'shell(git:*)' --deny-tool 'shell(git push)'` : allow all Git commands except `git push`
- `copilot -p "Fix the bug in main.js" --allow-all-tools` : execute a prompt directly without interactive mode, while allowing `copilot` to run any command

## Categories
version-control, shell

## Links
- More information: https://docs.github.com/en/copilot/concepts/agents/about-copilot-cli
- tldr page: https://tldr.inbrowser.app/pages/common/copilot
