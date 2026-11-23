# git checkout

Faz checkout de uma branch ou caminhos para uma árvore de trabalho.

## Usage
- `git checkout -b nome_da_branch` : cria e muda para uma nova branch
- `git checkout -b nome_da_branch referência` : cria e muda para uma nova branch com base em uma referência específica (branch, remoto/branch, etiqueta são exemplos de referências válidas)
- `git checkout nome_da_branch` : muda para uma branch local existente
- `git checkout -` : muda para uma branch previamente verificada
- `git checkout -t nome_remoto/nome_da_branch` : muda para uma branch remota existente
- `git checkout .` : descarta todas as alterações não preparadas no diretório atual (consulte `git reset` para mais comandos do tipo desfazer)
- `git checkout caminho/para/arquivo` : descarta alterações não preparadas em um determinado arquivo
- `git checkout nome_da_branch -- caminho/para/arquivo` : substitui um arquivo no diretório atual com a versão com commit em uma determinada branch

## Links
- More information: https://git-scm.com/docs/git-checkout
