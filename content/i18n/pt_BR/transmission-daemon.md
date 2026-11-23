# transmission-daemon

Daemon controlado com `transmission-remote` ou sua interface web.

## Usage
- `transmission-daemon` : inicia uma sessão headless `transmission`
- `transmission-daemon -c caminho/para/diretorio` : inicia e acompanha um diretório específico por novos torrents
- `transmission-daemon -d >caminho/para/arquivo.json` : despeja configurações do daemon em formato JSON
- `transmission-daemon -t -u usuario -v senha -p 9091 -a 127.0.0.1` : inicia com configurações específicas para a interface web

## Links
- More information: https://manned.org/transmission-daemon
