# docker compose down

Para e remove contêineres, redes, imagens e volumes criados por `docker compose up`.

## Usage
- `docker compose down` : para e remove todos os contêineres e redes
- `docker compose down --rmi all` : para e remove contêineres, redes e todas as imagens usadas pelos serviços
- `docker compose down --rmi local` : para e remove contêineres, redes e apenas imagens sem uma tag personalizada
- `docker compose down -v` : para e remove contêineres, redes e todos os volumes
- `docker compose down --remove-orphans` : para e remove tudo, incluindo contêineres órfãos
- `docker compose -f caminho/para/config down` : para e remove contêineres usando um arquivo Compose alternativo
- `docker compose down -t tempo_limite` : para e remove contêineres com um tempo limite personalizado em segundos
- `docker compose down --remove-orphans -v` : remove contêineres de serviços não definidos no arquivo Compose

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/down/
