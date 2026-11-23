# abbr

Gerencie abreviações para fish-shell. Palavras definidas pelo usuário são substituídas por frases longas assim que são digitadas.

## Usage
- `abbr -a nome_abreviacao comando orgumentos_comando` : adicione uma nova abreviação
- `abbr --rename nome_antigo novo_nome` : renomeia uma abreviação existente
- `abbr -e nome_abreviacao` : apaga uma abreviação existente
- `ssh nome_host abbr -s | source` : importa abreviações definidas em outro host via SSH

## Links
- More information: https://fishshell.com/docs/current/cmds/abbr.html
