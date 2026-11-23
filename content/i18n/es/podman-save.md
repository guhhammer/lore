# podman save

Guarda una imagen en un archivo o directorio local.

## Usage
- `podman save -o ruta/al/archivo.tar image:tag` : guarda una imagen en un archivo `.tar`
- `podman save image:tag >ruta/al/archivo.tar` : guarda una imagen en `stdout`
- `podman save image:tag | gzip|bzip2|xz|zstd|zstdchunked >ruta/al/archivo.tar.ext]` : guarda una imagen con compresión
- `podman save image:tag | zstd -T 0 --ultra | pv | ssh usuario@host_remoto podman load` : transfiere una imagen a un sistema remoto con compresión sobre la marcha y barra de progreso

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-save.1.html
