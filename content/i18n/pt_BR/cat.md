# cat

Exibe e concatena o conteúdo de arquivos.

## Usage
- `cat caminho/para/arquivo` : exibe o conteúdo de um arquivo na `stdout`
- `cat caminho/para/arquivo1 caminho/para/arquivo2 ... >caminho/para/arquivo_de_saída` : concatena o conteúdo de vários arquivos em um arquivo de saída
- `cat caminho/para/arquivo1 caminho/para/arquivo2 ... >>caminho/para/arquivo_de_saída` : anexa o conteúdo de vários arquivos ao final de um arquivo de saída
- `cat -u /dev/tty12 >/dev/tty13` : copia o conteúdo de um arquivo em um arquivo de saída sem armazenamento em buffer
- `cat - >caminho/para/arquivo` : escreve a `stdin` em um arquivo

## Links
- More information: https://manned.org/cat.1posix
