# docker compose start

Inicia contêineres existentes dos serviços.

## Usage
- `docker compose start` : inicia os contêineres existentes de todos os serviços
- `docker compose start serviço1 serviço2 ...` : inicia os contêineres existentes de um ou mais serviços
- `docker compose start --dry-run` : simula a inicialização dos contêineres existentes
- `docker compose start --wait` : inicia os contêineres existentes e aguarda até que os serviços estejam em execução ou saudáveis
- `docker compose start --wait --wait-timeout segundos` : inicia os contêineres existentes e aguarda por até um número especificado de segundos

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/start/
