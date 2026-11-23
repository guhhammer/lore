# git commit

Faz um commit dos arquivos no repositório.

## Usage
- `git commit -m "mensagem"` : faz um commit com os arquivos preparados no repositório com uma mensagem
- `git commit -F caminho/para/arquivo_de_mensagem_do_commit` : faz um commit com os arquivos preparados com uma mensagem lida de um arquivo
- `git commit -a -m "mensagem"` : prepara automaticamente todos os arquivos modificados e excluídos e faz o commit com uma mensagem
- `git commit -S id_da_chave -m "mensagem"` : faz um commit com os arquivos preparados e assina-os com a chave GPG especificada (ou a definida no arquivo de configuração se nenhum argumento for especificado)
- `git commit --amend` : atualiza o último commit adicionando as alterações atualmente preparadas, alterando o hash do commit
- `git commit caminho/para/arquivo1 caminho/para/arquivo2 ...` : faz um commit apenas de arquivos específicos (já preparados)
- `git commit -m "mensagem" --allow-empty` : cria um commit, mesmo se não haja arquivos preparados

## Links
- More information: https://git-scm.com/docs/git-commit
