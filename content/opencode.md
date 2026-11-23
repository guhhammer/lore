# opencode

An AI coding agent. Some subcommands such as `auth`, `models`, `web`, etc. have their own usage documentation.

## Usage
- `opencode` : start the interactive TUI
- `opencode -c` : continue the most recent session
- `opencode run "message"` : run opencode in non-interactive mode by passing a prompt directly
- `opencode run -m provider/model --agent agent_name "message"` : use a specific model and agent
- `opencode models` : list all available models from configured providers
- `opencode auth login` : manage credentials and login for providers
- `opencode serve -h hostname -p port` : start a headless opencode server for API access
- `opencode agent create` : create a new agent with custom configuration

## Categories
documents, monitoring, network

## Links
- More information: https://opencode.ai/docs/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/opencode
