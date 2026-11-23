# bzip2

Um compressor de arquivos que utiliza o algoritmo Burrows-Wheeler.

## Usage
- `bzip2 arquivo` : compacta um arquivo
- `bzip2 -d arquivo_compactado.bz2` : descompacta um arquivo
- `bzip2 -dc arquivo_compactado.bz2` : descompacta um arquivo exibindo o conteúdo no terminal
- `bzip2 -t caminho/para/arquivo_compactado.bz2` : testa a integridade de cada arquivo dentro do arquivo compactado
- `bzip2 -v caminho/para/arquivos_compactados.bz2` : exibe a taxa de compressão para cada arquivo processado com informações detalhadas
- `bzip2 -f caminho/para/arquivo_compactado.bz2` : descompacta um arquivo sobrescrevendo arquivos existentes
- `bzip2 -h` : mostra ajuda

## Links
- More information: https://manned.org/bzip2
