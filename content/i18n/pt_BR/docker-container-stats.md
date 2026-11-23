# docker container stats

Exibe estatísticas dinâmicas de uso de recursos dos containers.

## Usage
- `docker stats` : exibe estatísticas atualizadas de todos os containers em execução
- `docker stats nome_do_container` : exibe estatísticas atualizadas de uma lista separada por espaço dos containers
- `docker stats --format "\{\{.Name\}\}:\t\{\{.CPUPerc\}\}"` : altera o formato das colunas para exibir o uso da CPU em porcentagem
- `docker stats -a` : exibe estatísticas para todos os containers (tanto em execução como parados)
- `docker stats --no-stream` : desabilita estatísticas atualizadas e só exibe o status naquele momento

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/stats/
