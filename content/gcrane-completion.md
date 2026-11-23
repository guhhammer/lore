# gcrane completion

Generate the autocompletion script for gcrane for the specified shell. The available shells are Bash, fish, PowerShell, and Zsh.

## Usage
- `gcrane completion shell_name` : generate the autocompletion script for your shell
- `gcrane completion shell_name --no-descriptions` : disable completion descriptions
- `source <(gcrane completion bash/zsh)` : load completions in your current shell session (Bash/Zsh)
- `gcrane completion fish | source` : load completions in your current shell session (fish)
- `gcrane completion bash >/etc/bash_completion.d/gcrane` : load completions for every new session (Bash)
- `gcrane completion zsh >"${fpath[1]}/_gcrane"` : load completions for every new session (Zsh)
- `gcrane completion fish >~/.config/fish/completions/gcrane.fish` : load completions for every new session (fish)
- `gcrane completion shell_name -h` : display help

## Categories
shell, monitoring, development

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gcrane-completion
