# docker container logs

Imprime registros del contenedor.

## Usage
- `docker logs nombre_contenedor` : imprime registros de un contenedor
- `docker logs nombre_contenedor -f` : imprime registros y los sigue
- `docker logs nombre_contenedor -n 5` : imprime las últimas 5 líneas
- `docker logs nombre_contenedor -t` : imprime registros y los añade con marcas de tiempo
- `docker logs nombre_contenedor --until hora` : imprime registros desde un momento determinado de la ejecución del contenedor (por ejemplo, 23m, 10s, 2013-01-02T13:23:37)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/logs/
