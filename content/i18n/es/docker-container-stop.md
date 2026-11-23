# docker container stop

Detiene uno o varios contenedores en ejecución.

## Usage
- `docker stop contenedor` : detiene un contenedor de Docker
- `docker stop -s señal contenedor` : detiene un contenedor enviándole una señal específica
- `docker stop -t segundos contenedor` : detiene un contenedor, esperando un número específico de segundos antes de forzar su cierre
- `docker stop contenedor1 contenedor2 ...` : detiene uno o varios contenedores
- `docker stop --help` : muestra la ayuda

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/stop/
