# 7za

Compactador de arquivos com uma alta taxa de compressão. Semelhante to '7z', a principal diferença é que este suporta menos tipos de arquivamento/compressão.

## Usage
- `7za a caminho/para/ficheiro_compactado.7z caminho/para/ficheiro_ou_diretório` : compacta um ficheiro ou diretório
- `7za a caminho/para/ficheiro_encriptado.7z -ppalavra-passe -mhe on caminho/para/ficheiro_compactado.7z` : encripta um arquivo existente (incluindo os nomes dos ficheiros)
- `7za x caminho/para/ficheiro_compactado.7z` : descompacta um arquivo mantendo a estrutura de diretórios original
- `7za x caminho/para/ficheiro_compactado.7z -ocaminho/para/diretório` : descompacta um arquivo para uma diretório especificado pelo utilizador
- `7za x caminho/para/ficheiro_compactado.7z -so` : descompacta um arquivo para a saída padrão (`stdout`)
- `7za a -t7z|bzip2|gzip|lzip|tar|... caminho/para/ficheiro_compactado.7z caminho/para/ficheiro_ou_diretório` : compacta utilizando um tipo específico de arquivamento/compressão
- `7za l caminho/para/ficheiro_compactado.7z` : lista os conteúdos de um arquivo

## Links
- More information: https://manned.org/7za
