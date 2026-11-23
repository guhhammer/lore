# docker buildx du

Exibe o uso de disco de um builder.

## Usage
- `docker buildx du` : exibe o uso de disco
- `docker buildx du --filter "description~=golang"` : filtra a saída com base em uma condição específica
- `docker buildx du --verbose` : exibe uma saída detalhada
- `docker buildx du --format "table \{\{.ID\}\}    \{\{.Description\}\}"` : formata a saída usando um template Go
- `docker buildx du --format json | jq .` : exibe a saída formatada como JSON com o comando `jq`
- `docker buildx du --builder nome_do_builder` : exibe o uso de disco de um builder específico

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/du/
