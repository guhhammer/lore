# docker compose

Ejecuta y gestiona aplicaciones Docker multicontenedor.

## Usage
- `docker compose ps` : lista todos los contenedores en ejecución
- `docker compose up -d` : crea e inicia todos los contenedores en segundo plano usando un archivo `docker-compose.yml` desde el directorio actual
- `docker compose up --build` : inicia todos los contenedores, y se reconstruye si es necesario
- `docker compose -p nombre_proyecto -f ruta/al/archivo up` : inicia todos los contenedores especificando un nombre de proyecto y utilizando un archivo de composición alternativo
- `docker compose stop` : detiene todos los contenedores en ejecución
- `docker compose down --rmi all -v` : detiene y elimina todos los contenedores, redes, imágenes y volúmenes
- `docker compose logs -f` : sigue los registros de todos los contenedores
- `docker compose logs -f nombre_del_contenedor` : sigue los registros de un contenedor específico

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
