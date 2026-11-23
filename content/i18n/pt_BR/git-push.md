# git push

Envia commits para um repositório remoto.

## Usage
- `git push` : envia alterações locais na branch atual para sua contraparte remota padrão
- `git push nome_remoto branch_local` : envia alterações de uma branch local específica para sua contraparte remota
- `git push -u nome_remoto branch_local` : envia alterações de uma branch local específica para sua contraparte remota, e define a branch remota como o destino push/pull padrão da branch local
- `git push nome_remoto branch_local:branch_remota` : envia alterações de uma branch local específica para uma branch remota específica
- `git push --all nome_remoto` : envia alterações em todas as branches locais para suas contrapartes em um determinado repositório remoto
- `git push nome_remoto -d branch_remota` : exclui uma branch em um repositório remoto
- `git push --prune nome_remoto` : remove branches remotas que não tenham uma contraparte local
- `git push --tags` : publica etiquetas que ainda não estão no repositório remoto

## Links
- More information: https://git-scm.com/docs/git-push
