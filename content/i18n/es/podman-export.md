# podman export

Exporta el sistema de archivos de un contenedor y lo guarda como un archivo tar en el equipo local.

## Usage
- `podman export -o ruta/al/archivo.tar nombre_o_id_del_contenedor` : exporta el sistema de archivos de un contenedor a un archivo `.tar`
- `podman export nombre_o_id_del_contenedor >ruta/al/archivo.tar` : exporta el sistema de archivos de un contenedor a `stdout`

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-export.1.html
