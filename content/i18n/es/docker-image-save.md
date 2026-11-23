# docker image save

Exporta imágenes de Docker a un archivo.

## Usage
- `docker save imagen:etiqueta >ruta/a/archivo.tar` : guarda una imagen redirigiendo `stdout` a un archivo `.tar`
- `docker save -o ruta/al/archivo.tar imagen:etiqueta` : guarda una imagen en un archivo `.tar`
- `docker save -o ruta/al/archivo.tar nombre_de_la_imagen` : guarda todas las etiquetas de la imagen
- `docker save -o ruta/al/archivo.tar nombre_de_la_imagen:etiqueta1 nombre_de_la_imagen:etiqueta2 ...` : selecciona etiquetas concretas de una imagen para guardarlas

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/save/
