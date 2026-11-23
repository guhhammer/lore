# daemon

Roda processos em daemons.

## Usage
- `daemon -n "nome" comando` : roda um comando como um daemon
- `daemon -n "nome" -r comando` : roda um comando como um daemon que será reiniciado se o comando falhar
- `daemon -n "nome" -r -A 2 -L 10 comando` : roda um comando como um daemon que será reiniciado se falar, com duas tentativas a cada 10 segundos
- `daemon -n "nome" -l caminho/para/arquivo.log comando` : roda um comando como um daemon, gravando registros em um arquivo específico
- `daemon -n "nome" --stop` : elimina um daemon (`SIGTERM`)
- `daemon --list` : lista os daemons

## Links
- More information: https://manned.org/daemon.1
