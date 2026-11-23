# docker

Administra contenedores e imágenes de Docker. Algunos subcomandos, como `run`, tienen su propia documentación de uso.

## Usage
- `docker ps -a` : lista todos los contenedores de Docker (en ejecución y detenidos)
- `docker run --name nombre_de_contenedor imagen` : inicia un contenedor desde una imagen con un nombre personalizado
- `docker container start|stop nombre_de_contenedor` : inicia o detiene un contenedor existente
- `docker pull imagen` : descarga una imagen desde un registro de Docker
- `docker images` : muestra la lista de imágenes descargadas
- `docker exec -it nombre_de_contenedor sh` : inicia una línea de comandos dentro de un contenedor en ejecución
- `docker rm nombre_de_contenedor` : elimina un contenedor detenido
- `docker logs -f nombre_de_contenedor` : obtén y sigue los registros de un contenedor

## Links
- More information: https://docs.docker.com/reference/cli/docker/
