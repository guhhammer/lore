# docker volume

Gerenciar volumes do Docker.

## Usage
- `docker volume create nome_do_volume` : cria um volume
- `docker volume create --label rótulo nome_do_volume` : cria um volume com um rótulo específico
- `docker volume create -o type=tmpfs -o device=tmpfs -o o=size=100m,uid=1000 nome_do_volume` : cria um volume `tmpfs` com tamanho de 100 MiB e uid 1000
- `docker volume ls` : lista todos os volumes
- `docker volume rm nome_do_volume` : remove um volume
- `docker volume inspect nome_do_volume` : exibe informações sobre um volume
- `docker volume prune` : remove todos os volumes locais não utilizados
- `docker volume subcomando --help` : exibe ajuda para um subcomando

## Links
- More information: https://docs.docker.com/reference/cli/docker/volume/
