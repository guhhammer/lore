# cut

Recorta campos do `stdin` ou de arquivos.

## Usage
- `comando | cut --characters|fields 1|1,10|1-10|1-|-10` : imprime um intervalo específico de caracteres/campos de cada linha
- `comando | cut --delimiter "," --fields 1` : imprime um intervalo de campos de cada linha com um delimitador específico
- `cut --characters 1 caminho/para/arquivo` : imprime um intervalo de caracteres de cada linha de um arquivo específico
- `comando | cut --zero-terminated --fields 1` : imprime campos específicos de linhas terminadas em `NUL` (ex.: assim como em `find . -print0`) ao invés de novas linhas

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cut-invocation.html
