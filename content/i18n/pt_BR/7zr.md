# 7zr

Um compactador de arquivos com alta taxa de compressão. Similar ao `7z`, exceto que este suporta apenas arquivos no formato 7z.

## Usage
- `7zr a caminho/para/arquivo_compactado.7z caminho/para/arquivo_ou_diretorio` : compacta um arquivo ou diretório
- `7zr a arquivo_criptografado.7z -psenha -mhe=on caminho/para/arquivo_compactado.7z` : criptografa um arquivo existente (incluindo cabeçalhos)
- `7zr x arquivo_compactado.7z` : descompacta um arquivo mantendo a estrutura de diretórios original
- `7zr x caminho/para/arquivo_compactado.7z -ocaminho/para/diretorio` : descompacta um arquivo em um diretório especificado pelo usuário
- `7zr x caminho/para/arquivo_compactado.7z -so` : descompacta um arquivo para a saída padrão
- `7zr l caminho/para/arquivo_compactado.7z` : exibe o conteúdo de um arquivo
- `7zr a caminho/para/arquivo_compactado.7z -mx=0|1|3|5|7|9 caminho/para/diretorio` : define o nível de compressão (maior significa mais compressão, porém mais lento)

## Links
- More information: https://manned.org/7zr
