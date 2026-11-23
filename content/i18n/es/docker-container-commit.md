# docker container commit

Crea una nueva imagen a partir de los cambios de un contenedor.

## Usage
- `docker commit contenedor imagen:etiqueta` : crea una imagen a partir de un contenedor específico
- `docker commit -c "CMD comando" contenedor imagen:etiqueta` : aplica una instrucción `CMD` Dockerfile a la imagen creada
- `docker commit -c "ENV nombre=valor" contenedor imagen:etiqueta` : aplica una instrucción `ENV` Dockerfile a la imagen creada
- `docker commit -a "autor" contenedor imagen:etiqueta` : crea una imagen con un autor específico en los metadatos
- `docker commit -m "comentario" contenedor imagen:etiqueta` : crea una imagen con un comentario específico en los metadatos
- `docker commit -p false contenedor imagen:etiqueta` : crea una imagen sin pausar el contenedor durante la confirmación
- `docker commit --help` : muestra la ayuda

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/commit/
