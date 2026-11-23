# transmission-remote

Utilitário de controle remoto para `transmission-daemon` e `transmission`.

## Usage
- `transmission-remote hostname -a torrent|url -w /caminho/para/diretorio_download` : adiciona um arquivo torrent ou link magnético para o Transmission e baixa para um diretório específico
- `transmission-remote hostname -w /caminho/para/diretorio_download` : altera o diretório de download padrão
- `transmission-remote hostname -l` : lista todos os torrents
- `transmission-remote hostname -t "1,2" -s -t 3 -S` : inicia os torrents 1 e 2, interrompe o torrent 3
- `transmission-remote hostname -t 1 -r -t 2 -rad` : remove os torrents 1 e 2 e também exclui dados locais do torrent 2
- `transmission-remote hostname -t all -S` : interrompe todos os torrents
- `transmission-remote hostname -t "1-10,15-20" --move /caminho/para/nodo_diretorio` : move os torrents 1-10 e 15-20 para um novo diretório (que será criado se não existir)

## Links
- More information: https://manned.org/transmission-remote
