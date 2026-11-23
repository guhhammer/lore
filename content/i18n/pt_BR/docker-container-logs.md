# docker container logs

Exibe os logs dos containers.

## Usage
- `docker logs nome_do_container` : exibe logs de um container
- `docker logs nome_do_container -f` : exibe logs de um container e segue exibindo
- `docker logs nome_do_container -n 5` : exibe as últimas 5 linhas
- `docker logs nome_do_container -t` : exibe logs e adiciona a informação de hora ao log
- `docker logs nome_do_container --until tempo` : exibe logs até um certo ponto no tempo de execução do container (por exemplo: 23m, 10s, 2013-01-02T13:23:37)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/logs/
