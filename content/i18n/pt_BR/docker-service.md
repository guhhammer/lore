# docker service

Gerenciar os serviços em um daemon do Docker.

## Usage
- `docker service ls` : lista os serviços em um daemon do Docker
- `docker service create --name nome_do_serviço imagem:tag` : cria um novo serviço
- `docker service inspect nome_do_serviço|id` : exibe informações detalhadas de uma lista separada por espaços de serviços
- `docker service ps nome_do_serviço|id` : lista as tarefas de uma lista separada por espaços de serviços
- `docker service scale nome_do_serviço=quantidade_de_réplicas` : escala para um número específico de réplicas para uma lista separada por espaços de serviços
- `docker service rm nome_do_serviço|id` : remove uma lista separada por espaços de serviços

## Links
- More information: https://docs.docker.com/reference/cli/docker/service/
