# ect

Efficient Compression Tool. Otimizador de arquivos escrito em C++. Suporta arquivos do tipo PNG, JPEG, gzip and Zip.

## Usage
- `ect caminho/para/arquivo.png` : comprime um arquivo
- `ect -9 --mt-deflate caminho/para/arquivo.zip` : comprime um arquivo com level de compressão específico e multithreading (1=Mais rápido (pior), 9=Mais lento (Melhor). O padrão é 3)
- `ect -recurse caminho/para/diretório` : comprime todos os arquivos em um diretório recursivamente
- `ect -keep caminho/para/arquivo.png` : comprime um arquivo, mantendo o tempo de modificação original
- `ect -strip caminho/para/arquivo.png` : comprime um arquivo, removendo metadados

## Links
- More information: https://github.com/fhanau/Efficient-Compression-Tool
