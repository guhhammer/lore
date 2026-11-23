# docker container logs

Mostra i log di un container.

## Usage
- `docker logs nome_container` : mostra i log di un container
- `docker logs nome_container -f` : segui i log di un container
- `docker logs nome_container -n 5` : mostra le ultime 5 righe
- `docker logs nome_container -t` : mostra i log mettendo un timestamp in coda
- `docker logs nome_container --until momento` : mostra i log avvenuti prima di un dato momento nell'esecuzione del container (ad esempio, 23m, 10s, 2013-01-02T13:23:37)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/logs/
