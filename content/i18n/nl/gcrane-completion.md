# gcrane completion

Genereer het autocompletion script voor gcrane voor de opgegeven shell. De beschikbare shells zijn Bash, fish, PowerShell en Zsh.

## Usage
- `gcrane completion shell_naam` : genereer het autocompletion script voor je shell
- `gcrane completion shell_naam --no-descriptions` : zet de completion beschrijvingen uit
- `source <(gcrane completion bash/zsh)` : laad completions in je huidige shellsessie (Bash/Zsh)
- `gcrane completion fish | source` : laad completions in je huidige shellsessie (fish)
- `gcrane completion bash >/etc/bash_completion.d/gcrane` : laad completions voor elke nieuwe sessie (Bash)
- `gcrane completion zsh >"${fpath[1]}/_gcrane"` : laad completions voor elke nieuwe sessie (Zsh)
- `gcrane completion fish >~/.config/fish/completions/gcrane.fish` : laad completions voor elke nieuwe sessie (fish)
- `gcrane completion shell_naam -h` : toon de help
