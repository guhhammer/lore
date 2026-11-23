# git branch

Comando principal do Git para trabalhar com branches.

## Usage
- `git branch -a` : lista todas as branches (locais e remotas; a branch atual é destacada por `*`)
- `git branch -a --contains hash_do_commit` : lista quais branches incluem um commit específico do Git em seu histórico
- `git branch --show-current` : mostra o nome da branch atual
- `git branch nome_da_branch` : cria uma nova branch baseada no commit atual
- `git branch nome_da_branch hash_do_commit` : crua uma nova branch baseada em um commit específico
- `git branch -m antigo_nome_da_branch novo_nome_da_branch` : renomeia uma branch (não precisa fazer checkout para isso)
- `git branch -d nome_da_branch` : exclui a branch local (não precisa fazer checkout para isso)
- `git push nome_remoto -d nome_da_branch_remota` : exclui uma branch remota

## Links
- More information: https://git-scm.com/docs/git-branch
