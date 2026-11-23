# git remote

Gerencia repositórios monitorados ("remotes").

## Usage
- `git remote -v` : lista remotes existentes com seus nomes e URLs
- `git remote show nome_do_remote` : mostra infomação de um remote específico
- `git remote add nome_do_remote url_do_remote` : adiciona um remote
- `git remote set-url nome_do_remote nova_url` : muda a URL de um remote (use `--add` para manter a URL existente)
- `git remote get-url nome_do_remote` : mostra a URL de um remote
- `git remote remove nome_do_remote` : remove um remote
- `git remote rename nome_antigo novo_nome` : renomeia um remote

## Links
- More information: https://git-scm.com/docs/git-remote
