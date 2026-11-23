# podman load

Carga una imagen desde un archivo OCI o Docker creado con `podman save`.

## Usage
- `podman load -i ruta/al/archivo.tar` : carga una imagen desde un archivo `.tar`
- `podman load -i ruta/al/archivo.tar.ext` : carga una imagen desde un archivo `.tar` comprimido
- `podman load -q -i ruta/al/archivo.tar` : carga una imagen y muestra una salida silenciosa (solo muestra el ID de la imagen)
- `podman <ruta/al/archivo.tar load` : carga una imagen desde `stdin`

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-load.1.html
