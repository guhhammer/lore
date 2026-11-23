# 7za

Um compactador de arquivos com alta taxa de compressão. Similar ao `7z`, exceto que este suporta menos tipos de arquivo mas é multiplataforma.

## Usage
- `7za a caminho/para/arquivo_compactado.7z caminho/para/arquivo_ou_diretório` : compacta um arquivo ou diretório
- `7za a caminho/para/arquivo_criptografado.7z -psenha -mhe=on caminho/para/arquivo_compactado.7z` : criptografa um arquivo existente (incluindo cabeçalhos)
- `7za x caminho/para/arquivo_compactado.7z` : descompacta um arquivo mantendo a estrutura de diretórios original
- `7za x caminho/para/arquivo_compactado.7z -ocaminho/para/diretório` : descompacta um arquivo em um diretório especificado pelo usuário
- `7za x caminho/para/arquivo_compactado.7z -so` : descompacta um arquivo para a saída padrão
- `7za a -tzip|gzip|bzip2|tar|... caminho/para/arquivo_compactado.7z caminho/para/arquivo_ou_diretório` : compacta utilizando um tipo específico de arquivamento/compressão
- `7za l caminho/para/arquivo_compactado.7z` : exibe o conteúdo de um arquivo
- `7za a caminho/para/arquivo_compactado.7z -mx=0|1|3|5|7|9 caminho/para/arquivo_ou_diretório` : define o nível de compressão (maior significa mais compressão, porém mais lento)

## Links
- More information: https://manned.org/7za
