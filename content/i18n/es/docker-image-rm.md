# docker image rm

Elimina imágenes de Docker.

## Usage
- `docker rmi imagen1 imagen2 ...` : elimina una o varias imágenes por su nombre
- `docker rmi -f imagen` : fuerza la eliminación de una imagen
- `docker rmi --no-prune imagen` : elimina una imagen sin borrar las imágenes parentales sin etiquetar
- `docker rmi --help` : muestra la ayuda

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/rm/
