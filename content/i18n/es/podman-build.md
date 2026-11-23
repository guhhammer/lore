# podman build

Herramienta que no corre como servicio (daemon) para construir imágenes de contenedores.

## Usage
- `podman build ruta/al/directorio` : crea una imagen usando un `Dockerfile` o `Containerfile` en el directorio especificado
- `podman build -t nombre_de_la_imagen:version ruta/al/directorio` : crea una imagen con una etiqueta especificada
- `podman build -f Archivo_contenedor.different .` : crea una imagen de un archivo no estándar
- `podman build --no-cache ruta/al/directorio` : crea una imagen sin usar ninguna imagen previamente almacenada en caché
- `podman build -q ruta/al/directorio` : crea una imagen suprimiendo cualquier mensaje informativo (output)

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-build.1.html
