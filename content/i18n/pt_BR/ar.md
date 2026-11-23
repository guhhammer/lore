# ar

Cria, modifica e extrai de arquivos Unix. Normalmente usado para bibliotecas estáticas (`.a`) e pacotes Debian (`.deb`).

## Usage
- `ar x caminho/para/arquivo.a` : descompacta todos os membros de um arquivo compactado
- `ar t caminho/para/arquivo.ar` : lista o conteúdo em um arquivo compactado específico
- `ar r caminho/para/arquivo.deb caminho/para/binário-debian caminho/para/control.tar.gz caminho/para/data.tar.xz ...` : substitui ou adiciona arquivos específicos para um arquivo compactado
- `ar s caminho/para/arquivo.a` : insere um índice de arquivos objetos (equivalente a usar `ranlib`)
- `ar rs caminho/para/arquivo.a caminho/para/arquivo1.o caminho/para/arquivo2.o ...` : cria um arquivo compactado com arquivos específicos, acompanhado por um índice de arquivo objeto

## Links
- More information: https://manned.org/ar
