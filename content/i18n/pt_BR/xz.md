# xz

Compactar ou descompactar arquivos XZ ou LZMA.

## Usage
- `xz caminho/para/arquivo` : compacta um arquivo no formato xz
- `xz -d caminho/para/arquivo.xz` : descompacta um arquivo no formato xz
- `xz -F lzma caminho/para/arquivo` : compacta um arquivo no formato LZMA
- `xz -d -F lzma caminho/para/arquivo.lzma` : descompacta um arquivo no formato LZMA
- `xz -d -c caminho/para/arquivo.xz` : descompacta um arquivo e escrever a saída no terminal (implica `--keep`)
- `xz -k caminho/para/arquivo` : compacta um arquivo sem apagar o arquivo original
- `xz -0 caminho/para/arquivo` : compacta um arquivo utilizando a compactação mais rápida
- `xz -9 caminho/para/arquivo` : compacta um arquivo utilizando a compactação mais eficiente

## Links
- More information: https://manned.org/xz
