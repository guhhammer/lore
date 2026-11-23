# docker container run

Ejecuta un comando en un nuevo contenedor Docker.

## Usage
- `docker run imagen:etiqueta comando` : ejecuta un comando en un nuevo contenedor desde una imagen etiquetada
- `docker run -d imagen comando` : ejecuta un comando en un nuevo contenedor en segundo plano y muestra su ID
- `docker run --rm -it imagen comando` : ejecuta un comando en un contenedor único en modo interactivo y pseudo-TTY
- `docker run -e 'variable=valor' -e variable imagen comando` : ejecuta el comando en un nuevo contenedor con las variables de entorno pasadas
- `docker run -v /ruta/a/host_path:/ruta/al/contenedor imagen comando` : ejecuta comando en un nuevo contenedor con volúmenes montados vinculados
- `docker run -p puerto_host:puerto_contenedor imagen comando` : ejecuta comando en un nuevo contenedor con puertos publicados
- `docker run --entrypoint comando imagen` : ejecuta comando en un nuevo contenedor sobrescribiendo el punto de entrada de la imagen
- `docker run --network red imagen` : ejecuta comando en un nuevo contenedor conectándolo a una red

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/run/
