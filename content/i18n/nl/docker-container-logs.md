# docker container logs

Toon container logs.

## Usage
- `docker logs naam|id` : toon logs van een container
- `docker logs naam|id -f` : toon logs en volg deze
- `docker logs naam|id -n 5` : toon de laatste 5 regels
- `docker logs naam|id -t` : toon logs en voorzie deze van timestamps
- `docker logs naam|id --until time` : toon logs vanaf een bepaald tijdstip van de uitvoering van de container (bijv. 23m, 10s, 2013-01-02T13:23:37)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/logs/
