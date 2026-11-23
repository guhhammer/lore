# podman-compose

Ejecuta y gestiona la definición del contenedor según la especificación de composición (Compose Specification).

## Usage
- `podman-compose ps` : lista todos los contenedores en funcionamiento
- `podman-compose up -d` : crea e inicia todos los contenedores en segundo plano utilizando un `docker-compose.yml` local
- `podman-compose up --build` : inicia todos los contenedores, construyendo si es necesario
- `podman-compose -f ruta/al/archivo.yaml up` : inicia todos los contenedores usando un archivo de composición alterno
- `podman-compose stop` : detiene todos los contenedores en funcionamiento
- `podman-compose down -v` : quita todos los contenedores, redes y volúmenes
- `podman-compose logs -f nombre_del_contenedor` : sigue los registros de un contenedor (omite todos los nombres de los contenedores)
- `podman-compose run nombre_del_servicio comando` : ejecuta un comando de una sola vez en un servicio sin puertos mapeados

## Links
- More information: https://github.com/containers/podman-compose
