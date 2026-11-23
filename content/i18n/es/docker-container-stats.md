# docker container stats

Muestra una transmisión en vivo de las estadísticas de uso de recursos de los contenedores.

## Usage
- `docker stats` : muestra una transmisión en vivo de las estadísticas de todos los contenedores en ejecución
- `docker stats contenedor1 contenedor2 ...` : muestra una transmisión en vivo de las estadísticas de uno o más contenedores
- `docker stats --format "\{\{.Name\}\}:\t\{\{.CPUPerc\}\}"` : cambia el formato de las columnas para mostrar el porcentaje de uso de la CPU del contenedor
- `docker stats -a` : muestra estadísticas de todos los contenedores (tanto en ejecución como detenidos)
- `docker stats --no-stream` : desactiva las estadísticas en tiempo real y solo extrae las estadísticas actuales

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/stats/
