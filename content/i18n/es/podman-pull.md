# podman pull

Descarga imágenes de un registro de contenedores.

## Usage
- `podman pull imagen:etiqueta` : descarga una imagen de contenedor específica
- `podman pull -q imagen:etiqueta` : descarga una imagen de contenedor en modo silencioso
- `podman pull -a imagen` : descarga todas las etiquetas de una imagen de contenedor
- `podman pull --platform linux/arm64 imagen:etiqueta` : descarga una imagen de contenedor para una plataforma específica
- `podman pull --tls-verify=false imagen:etiqueta` : descarga una imagen de contenedor sin verificación TLS
- `podman pull -h` : muestra la ayuda

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-pull.1.html
