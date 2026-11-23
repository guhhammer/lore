# docker compose logs

Exibe a saída dos contêineres de uma aplicação Docker Compose.

## Usage
- `docker compose logs` : exibe os registros (logs) de todos os serviços
- `docker compose logs nome_do_serviço` : exibe os registros de um serviço específico
- `docker compose logs -f` : exibe os registros e acompanha novas saídas (como `tail --follow`)
- `docker compose logs -t` : exibe os registros com timestamps
- `docker compose logs -n n` : exibe apenas as últimas `n` linhas de registros de cada contêiner
- `docker compose logs --since timestamp` : exibe os registros a partir de um horário específico
- `docker compose logs --until timestamp` : exibe os registros até um horário específico
- `docker compose logs serviço1 serviço2 ...` : exibe os registros de vários serviços específicos

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/logs/
