# kubectl completion

Генерировать код автодополнения для команд `kubectl`.

## Usage
- `kubectl completion bash|zsh|fish|powershell` : вывести скрипт автодополнения для Bash, Zsh, fish или PowerShell
- `source <(kubectl completion bash|zsh)` : загрузить автодополнение Bash или Zsh в текущую сессию оболочки
- `kubectl completion bash >>~/.bashrc` : добавить скрипт автодополнения Bash в `~/.bashrc`
- `kubectl completion zsh >"${fpath[1]}/_kubectl"` : записать скрипт автодополнения Zsh в файл в `fpath`
- `kubectl completion fish | source` : загрузить автодополнение fish в текущую сессию оболочки
- `kubectl completion fish >~/.config/fish/completions/kubectl.fish` : сохранить автодополнение fish
- `kubectl completion powershell | Out-String | Invoke-Expression` : загрузить автодополнение PowerShell в текущую сессию оболочки
- `kubectl completion powershell >>$PROFILE` : сохранить автодополнение PowerShell

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_completion/
