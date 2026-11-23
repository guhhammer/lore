# join

Junta linhas de dois arquivos ordenados em um campo comum.

## Usage
- `join caminho/para/arquivo1 caminho/para/arquivo2` : junta dois arquivos no primeiro campo (padrão)
- `join -t ',' caminho/para/arquivo1 caminho/para/arquivo2` : junta dois arquivos usando uma vírgula (em vez de um espaço) como separador de campo
- `join -1 3 -2 1 caminho/para/arquivo1 caminho/para/arquivo2` : junta campo3 do arquivo1 ao campo1 do arquivo2
- `join -a 1 caminho/para/arquivo1 caminho/para/arquivo2` : produz uma linha para cada linha que não pode ser pareada para o arquivo1
- `cat caminho/para/arquivo1 | join - caminho/para/arquivo2` : junta aquivo da entrada padrão (`stdin`)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/join-invocation.html
