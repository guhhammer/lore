# transmission-cli

Um cliente BitTorrent leve e de linha de comando. Esta ferramenta foi descontinuada. Por favor, veja `transmission-remote`.

## Usage
- `transmission-cli url|magnet|caminho/para/arquivo` : baixa um torrent específico
- `transmission-cli -w caminho/para/diretório_download url|magnet|caminho/para/arquivo` : baixa um torrent para um diretório específico
- `transmission-cli --new caminho/para/arquivo_ou_diretório_origem` : cria um arquivo torrent de um arquivo ou diretório específico
- `transmission-cli -d 50 url|magnet|caminho/para/arquivo` : especifica o limite de velocidade de download (em KB/s)
- `transmission-cli -u 50 url|magnet|caminho/para/arquivo` : especifica o limite de velocidade de upload (em KB/s)
- `transmission-cli -p número_porta url|magnet|caminho/para/arquivo` : usa uma porta específica para conexões
- `transmission-cli -er url|magnet|caminho/para/arquivo` : força criptografia para conexões com pares
- `transmission-cli -b url_lista_bloqueio|caminho/para/lista_bloqueio url|magnet|caminho/para/arquivo` : usa uma lista de bloqueio de pares formatados em Bluetack

## Links
- More information: https://manned.org/transmission-cli
