# podman commit

Crea una nueva imagen basada en el contenedor modificado.

## Usage
- `podman commit contenedor imagen:etiqueta` : crea una imagen a partir de un contenedor específico
- `podman commit -c "ENV nombre=valor" contenedor imagen:etiqueta` : aplica una instrucción `ENV` a la imagen creada
- `podman commit -c CMD=comando -c ENTRYPOINT=comando -c "LABEL clave=valor" contenedor imagen:etiqueta` : aplicar las instrucciones `LABEL`, `ENTRYPOINT` y `CMD` a la imagen creada
- `podman commit -a "autor" -m "comentario" contenedor imagen:etiqueta` : crea una imagen con un autor y un comentario específicos en los metadatos
- `podman commit -p false contenedor imagen:etiqueta` : crea una imagen sin pausar el contenedor durante la confirmación
- `podman commit --help` : muestra la ayuda

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-commit.1.html
