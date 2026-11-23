# podman run

Ejecuta un comando en un nuevo contenedor Podman.

## Usage
- `podman run imagen:tag comando` : ejecuta el comando en un nuevo contenedor de una imagen etiquetada
- `podman run -d imagen:tag comando` : ejecuta el comando en un nuevo contenedor en el fondo y muestra su ID
- `podman run --rm -it imagen:tag comando` : ejecuta el comando en un contenedor único en modo interactivo y pseudo-TTY
- `podman run -e 'variable=valor' -e variable imagen:tag comando` : ejecuta el comando en un nuevo contenedor con variables de entorno enviadas
- `podman run -v /ruta/a/ruta_del_host:/ruta/a/ruta_del_contenedor imagen:tag comando` : ejecuta el comando en un nuevo contenedor con volúmenes montados en un contenedor
- `podman run -p puerto_del_anfitrion:puerto_del_contenedor imagen:tag comando` : ejecuta comando en un nuevo contenedor con los puertos publicados
- `podman run --entrypoint comando imagen:tag` : ejecuta el comando en un nuevo contenedor sobrescribiendo el punto de entrada (entrypoint) de la imagen
- `podman run --network red imagen:tag` : ejecuta el comando en un nuevo contenedor que lo conecta a una red

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-run.1.html
