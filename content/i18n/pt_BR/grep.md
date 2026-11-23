# grep

Acha padrões em arquivos usando expressões regulares.

## Usage
- `grep "padrão_pesquisado" caminho/para/arquivo` : pesquisa por um padrão em um arquivo
- `grep -F "string_exata" caminho/para/arquivo` : pesquisa por uma string exata (desabilita expressões regulares)
- `grep -rnI "padrão_pesquisado" caminho/para/diretório` : pesquisa por um padrão em todos os arquivos recursivamente em um diretório, mostrando o número das linhas das correspondências, ignorando arquivos binários
- `grep -Ei "padrão_pesquisado" caminho/para/arquivo` : usa expressões regulares estendidas (suporta `?`, `+`, `{}`, `()` and `|`), no modo insensível a maiúsculas e minúsculas
- `grep --context|--before-context|--after-context 3 "padrão_pesquisado" caminho/para/arquivo` : imprime 3 linhas de contexto em volta, antes ou depois de cada correspondência
- `grep -Hn --color=always "padrão_pesquisado" caminho/para/arquivo` : imprime o nome do arquivo e o número da linha para cada correspondência
- `grep -o "padrão_pesquisado" caminho/para/arquivo` : pesquisa por linhas que correspondem a um padrão, imprimindo apenas o texto correspondido
- `cat caminho/para/arquivo | grep -v "padrão_pesquisado"` : pesquisa `stdin` para linhas que não correspondem a um padrão

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html
