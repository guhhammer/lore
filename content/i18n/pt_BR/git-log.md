# git log

Mostra um histórico de commits.

## Usage
- `git log` : mostra a sequência de commits a partir do atual, em ordem cronológica reverse do repositório Git no diretório de trabalho atual
- `git log -p caminho/para/arquivo_ou_diretório` : mostra o histórico de um arquivo ou diretório determinado, incluindo diferenças
- `git log --stat` : mostra uma visão geral do(s) arquivo(s) alterado(s) em cada commit
- `git log --oneline --graph` : mostra um grafo dos commits no branch atual usando apenas a primera linha de cada mensagem de commit
- `git log --oneline --decorate --all --graph` : mostra um grafo de todos os commits, etiquetas e branches em todo o repositório
- `git log -i --grep cadeia_de_caracteres_para_pesquisa` : mostra apenas os commits cujas mensagem incluem uma determinada cadeia de caracteres (sem distinção entre maiúsculas e minúsculas)
- `git log -n número --author "autor"` : mostra os últimos N commits de um determinado autor
- `git log --before "2017-01-29" --after "2017-01-17"` : mostra os commits entre duas datas(aaaa-mm-dd)

## Links
- More information: https://git-scm.com/docs/git-log
