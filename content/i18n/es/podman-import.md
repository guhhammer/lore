# podman import

Importa un archivo tar y lo guarda como una imagen del sistema de archivos.

## Usage
- `podman import ruta/al/archivo.tar imagen:etiqueta` : importa un archivo tar desde un archivo local y crea una imagen
- `podman import https://example.com/imagen.tar imagen:etiqueta` : importa un archivo tar desde una URL
- `podman import -m "mensaje_de_confirmación" ruta/al/archivo.tar imagen:etiqueta` : importa un archivo tar y añade un mensaje de confirmación
- `podman import -c CMD=/bin/bash ruta/al/archivo.tar imagen:etiqueta` : importa un archivo tar y establece un comando por defecto (necesario para ejecutar el contenedor)

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-import.1.html
