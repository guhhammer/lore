# docker network

Criar e gerenciar redes do Docker.

## Usage
- `docker network ls` : lista todas as redes disponíveis e configuradas no daemon do Docker
- `docker network create -d nome_do_driver nome_da_rede` : cria uma rede definida pelo usuário
- `docker network inspect nome_da_rede` : exibe informações detalhadas de uma lista separada por espaços de redes
- `docker network connect nome_da_rede nome_do_contêiner|id` : conecta um contêiner a uma rede usando um nome ou ID
- `docker network disconnect nome_da_rede nome_do_contêiner|id` : desconecta um contêiner de uma rede
- `docker network prune` : remove todas as redes não utilizadas (que não são referenciadas por nenhum contêiner)
- `docker network rm nome_da_rede` : remove uma lista separada por espaços de redes não utilizadas

## Links
- More information: https://docs.docker.com/reference/cli/docker/network/
