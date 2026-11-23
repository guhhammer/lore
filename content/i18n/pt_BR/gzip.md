# gzip

Compacta/descompacta arquivos com compressão gzip (LZ77).

## Usage
- `gzip caminho/para/arquivo` : compacta um arquivo, substituindo-o por uma versão compactada gzip
- `gzip -d caminho/para/arquivo.gz` : descompacta um arquivo, substituindo-o pela versão descompactada original
- `gzip -k caminho/para/arquivo` : compacta um arquivo, mantendo o arquivo original
- `gzip -c caminho/para/arquivo >caminho/para/arquivo_compactado.gz` : compacta um arquivo definindo o nome do arquivo de saída
- `gzip -c -d caminho/para/arquivo.gz >caminho/para/arquivo_descompactado` : descompacta um arquivo gzip definindo o nome do arquivo de saída
- `gzip -1..9 -c caminho/para/arquivo >caminho/para/arquivo_compactado.gz` : especifica o nível de compactação. 1 é o mais rápido (baixa compressão), 9 é o mais lento (baixa compressão), o nível padrão é 6
- `gzip -v -d caminho/para/arquivo.gz` : mostra o nome e o percentual de redução para cada arquivo comprimido ou descomprimido

## Links
- More information: https://www.gnu.org/software/gzip/manual/gzip.html
