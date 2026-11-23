# gh copilot

Interact with GitHub Copilot. Note: This command is deprecated, use `copilot` instead.

## Usage
- `gh copilot suggest "Install git"` : suggest a command, given a description
- `gh copilot suggest -t git "Undo the most recent local commits"` : suggest a Git command
- `gh copilot explain "traceroute example.com"` : explain a command
- `echo 'eval "$(gh copilot alias -- bash)"' >>~/.bashrc` : generate shell-specific aliases for Bash
- `echo 'eval "$(gh copilot alias -- zsh)"' >>~/.zshrc` : generate shell-specific aliases for Zsh
- `gh copilot config` : configure options

## Categories
shell, version-control

## Links
- More information: https://github.com/github/gh-copilot#usage
- tldr page: https://tldr.inbrowser.app/pages/common/gh-copilot
