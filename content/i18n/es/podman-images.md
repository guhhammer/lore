# podman images

Gestiona imágenes de contenedores OCI/Docker.

## Usage
- `podman images` : lista todas las imágenes de contenedores
- `podman images -a` : lista todas las imágenes de contenedores incluyendo intermedias
- `podman images -q` : lista en modo silencioso (sólo ID numérico)
- `podman images -f dangling=true` : lista todas las imágenes no utilizadas por ningún contenedor
- `podman images "*imagen|imagen*"` : lista las imágenes que contienen una subcadena en su nombre

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-images.1.html
