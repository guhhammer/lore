# podman

Herramienta de gestión sencilla para pods, contenedores e imágenes. Podman proporciona una línea de comando comparable con Docker-CLI. En pocas palabras: `alias docker=podman`.

## Usage
- `podman ps -a` : lista todos los contenedores (ambos en funcionamiento y detenidos)
- `podman run --name nombre_del_contenedor imagen` : crea un contenedor desde una imagen con un nombre personalizado
- `podman start|stop nombre_del_contenedor` : inicia o detiene un contenedor existente
- `podman pull imagen` : extrae una imagen de un registro (Docker Hub predeterminado)
- `podman images` : muestra la lista de imágenes ya descargadas
- `podman exec -it nombre_del_contenedor sh` : abre una interfaz de comando dentro de un contenedor ya en funcionamiento
- `podman rm nombre_del_contenedor` : quita un contenedor detenido
- `podman logs -f nombre_del_contenedor id_contenedor ...` : muestra los registros de uno o más contenedores y muestra el registro (log)

## Links
- More information: https://docs.podman.io/en/latest/Commands.html
