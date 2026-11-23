# docker image ls

Muestra una lista de imágenes de Docker.

## Usage
- `docker images` : muestra una lista de todas las imágenes de Docker
- `docker images -a` : muestra una lista de todas las imágenes de Docker, incluidas las intermedias
- `docker images -q` : muestra la salida en modo silencioso (solo ID numéricos)
- `docker images -f dangling=true` : muestra todas las imágenes de Docker que no utiliza ningún contenedor
- `docker images "*nombre*"` : muestra las imágenes que contengan una subcadena en su nombre
- `docker images --format "\{\{.ID\}\}\t\{\{.Size\}\}\t\{\{.Repository\}\}:\{\{.Tag\}\}" | sort -k 2 -h` : ordena las imágenes por tamaño

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/ls/
