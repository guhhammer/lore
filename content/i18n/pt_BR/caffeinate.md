# caffeinate

Evita que o macOS entre em suspensão (repouso).

## Usage
- `caffeinate -u -t 3600` : evita a suspensão por uma hora (3600 segundos)
- `caffeinate -s "comando"` : evita a suspensão até que um comando seja concluído
- `caffeinate -w pid` : evita a suspensão até que um processo com o PID especificado seja concluído
- `caffeinate -i` : evita a suspensão (use `<Ctrl c>` para sair)
- `caffeinate -m` : evita a suspensão do disco (use `<Ctrl c>` para sair)

## Links
- More information: https://keith.github.io/xcode-man-pages/caffeinate.8.html
