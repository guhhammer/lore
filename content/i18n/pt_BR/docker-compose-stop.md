# docker compose stop

Para contêineres em execução sem removê-los.

## Usage
- `docker compose stop` : para todos os serviços em execução
- `docker compose stop serviço_1 serviço_2 ...` : para serviços específicos
- `docker compose stop -t segundos` : para com um tempo limite de desligamento personalizado em segundos
- `docker compose -f caminho/para/arquivo_compose stop` : para os serviços definidos em um arquivo compose específico
- `docker compose stop --dry-run` : executa de forma simulada (mostra as operações sem executá-las)
- `docker compose stop -t segundos serviço_1 serviço_2 ...` : para serviços específicos com um tempo limite personalizado

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/stop
