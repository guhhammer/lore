# kubectl completion

Generate shell completion code for `kubectl` commands.

## Usage
- `kubectl completion bash|zsh|fish|powershell` : print the completion script for Bash, Zsh, fish, or PowerShell
- `source <(kubectl completion bash|zsh)` : load Bash or Zsh completions into the current shell session
- `kubectl completion bash >>~/.bashrc` : append Bash completion script to `~/.bashrc`
- `kubectl completion zsh >"${fpath[1]}/_kubectl"` : write Zsh completion script to a file in the `fpath`
- `kubectl completion fish | source` : load fish completions into the current shell session
- `kubectl completion fish >~/.config/fish/completions/kubectl.fish` : persist fish completions
- `kubectl completion powershell | Out-String | Invoke-Expression` : load PowerShell completions into the current shell session
- `kubectl completion powershell >>$PROFILE` : persist PowerShell completions

## Categories
shell, development, monitoring

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_completion/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-completion
