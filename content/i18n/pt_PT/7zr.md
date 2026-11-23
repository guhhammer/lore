# 7zr

Compactador de arquivos com uma alta taxa de compressão. Semelhante ao `7z`, com a diferença que suporta apenas ficheiros `.7z`.

## Usage
- `7zr a caminho/para/arquivo_compactado.7z caminho/para/ficheiro_ou_diretório` : adiciona um ficheiro ou diretório a um novo ou existente arquivo
- `7zr a caminho/para/ficheiro_encriptado.7z -ppalavra-passe -mhe=on caminho/para/arquivo_compactado.7z` : encripa um arquivo existente (incluindo filenames)
- `7zr x caminho/para/arquivo_compactado.7z` : descompacta um arquivo mantendo a estrutura de diretórios original
- `7zr x caminho/para/arquivo_compactado.7z -ocaminho/para/diretório` : descompacta um arquivo para um diretório especificado pelo utilizador
- `7zr x caminho/para/arquivo_compactado.7z -so` : descompacta um arquivo para a saída padrão (`stdout`)
- `7zr l caminho/para/arquivo_compactado.7z` : lista os conteúdos de um arquivo

## Links
- More information: https://manned.org/7zr
