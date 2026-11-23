# git tag

Cria, exibe, exclui ou verifica tags. Uma tag é uma referência estática para um commit.

## Usage
- `git tag` : exibe todas as tags
- `git tag nome_da_tag` : cria uma tag com o nome fornecido apontando para o commit atual
- `git tag nome_da_tag commit` : cria uma tag com o nome fornecido apontando para um determinado commit
- `git tag nome_da_tag -m mensagem_da_tag` : cria uma tag anotada com a mensagem fornecida
- `git tag -d nome_da_tag` : exclui a tag com o nome fornecido
- `git fetch -t` : obtém tags atualizadas do remote
- `git push origin tag nome_da_tag` : envia uma tag para o remote
- `git tag --contains commit` : lista todas as tags cujos ancestrais incluem um determinado commit

## Links
- More information: https://git-scm.com/docs/git-tag
