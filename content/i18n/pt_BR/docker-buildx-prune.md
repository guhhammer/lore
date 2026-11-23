# docker buildx prune

Remove o cache de build.

## Usage
- `docker buildx prune` : remove o cache de build do builder ativo atual
- `docker buildx prune --filter "type=source.local"` : remove os registros de cache com base em um filtro específico
- `docker buildx prune --max-used-space 128mb` : remove os registros de cache usados menos recentemente até que o tamanho do cache fique abaixo de um limite específico
- `docker buildx prune --reserved-space 2gb` : remove os registros de cache usados menos recentemente até que uma quantidade específica de espaço livre em disco esteja disponível

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/prune/
