# hdiutil

Utilitário para criar e gerenciar imagens de disco.

## Usage
- `hdiutil attach caminho/para/aquivo_de_imagem` : monta uma imagem
- `hdiutil detach /Volumes/nome_do_volume` : desmonta uma imagem
- `hdiutil info` : lista as imagens montadas
- `hdiutil makehybrid -o caminho/para/arquivo_de_saída caminho/para/diretório` : cria uma imagem ISO a partir do conteúdo de um diretório

## Links
- More information: https://keith.github.io/xcode-man-pages/hdiutil.1.html
