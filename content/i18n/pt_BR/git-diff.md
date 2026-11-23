# git diff

Mostra alterações nos arquivos rastreados.

## Usage
- `git diff` : mostra as alterações não preparadas
- `git diff HEAD` : mostra todas as alterações sem commit (incluindo as preparadas)
- `git diff --staged` : mostra apenas as alterações preparadas (adicionadas, mas ainda sem commit)
- `git diff 'HEAD@{3 months|weeks|days|hours|seconds ago}'` : mostra as alterações de todos os commits desde uma determinada data/hora (uma expressão de data, por exemplo, "1 week 2 days" ou uma data ISO)
- `git diff --stat commit` : mostra estatísticas de comparação, como arquivos alterados, histogramas e número total de inserções/exclusões de linha
- `git diff --summary commit` : emite um resumo das criações de arquivos, renomeações e alterações de modo desde um determinado commit
- `git diff branch_1..branch_2 caminho/para/arquivo` : compara um único arquivo entre duas branches ou commits
- `git diff branch:caminho/para/arquivo2 caminho/para/arquivo1` : compara diferentes arquivos da branch atual com outra branch

## Links
- More information: https://git-scm.com/docs/git-diff
