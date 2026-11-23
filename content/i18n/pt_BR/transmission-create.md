# transmission-create

Cria arquivos BitTorrent `.torrent`.

## Usage
- `transmission-create -o caminho/para/exemplo.torrent -t url_anuncio_tracker -s 2048 caminho/para/arquivo_ou_diretório` : cria um torrent com 2048 KB como o tamanho da parte
- `transmission-create -p -o caminho/para/exemplo.torrent -t url_anuncio_tracker -s 2048 caminho/para/arquivo_ou_diretório` : cria um torrent privado com um tamanho de parte de 2048 KB
- `transmission-create -o caminho/para/exemplo.torrent -t url_rastreador1 -c comentário caminho/para/arquivo_ou_diretório` : cria um torrent com um comentário
- `transmission-create -o caminho/para/exemplo.torrent -t url_rastreador1 -t url_rastreador2 caminho/para/arquivo_ou_diretório` : cria um torrent com vários rastreadores
- `transmission-create -h` : mostra ajuda

## Links
- More information: https://manned.org/transmission-create
