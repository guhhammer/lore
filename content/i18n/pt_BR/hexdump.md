# hexdump

Imprime dados no formato ASCII, decimal, hexadecimal ou octal.

## Usage
- `hexdump caminho/para/arquivo` : imprime a representação hexadecimal de um arquivo, substituindo linhas duplicadas por '*'
- `hexdump -C caminho/para/arquivo` : imprime a representação hexadecimal e ASCII de um arquivo, em duas colunas
- `hexdump -C -n numero_de_bytes caminho/para/arquivo` : imprime a representação hexadecimal de um arquivo, porém apresentando apenas seus n primeiros bytes
- `hexdump -v caminho/para/arquivo` : imprime a representação hexadecimal completa de um arquivo (sem omitir linhas duplicadas)

## Links
- More information: https://manned.org/hexdump
