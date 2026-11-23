# look

Exibe linhas começando com um prefixo em um arquivo ordenado.

## Usage
- `look prefixo caminho/para/arquivo` : busca por linhas começando com um prefixo específico em um arquivo específico
- `look -f -d prefixo caminho/para/arquivo` : busca sem distinção entre maiúsculas e minúsculas apenas em caracteres alfanuméricos
- `look -t ,` : especifica um caractere de término de string (espaço por padrão)
- `look prefixo` : busca em `/usr/share/dict/words` (`--ignore-case` e `--alphanum` são assumidos)

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=look
