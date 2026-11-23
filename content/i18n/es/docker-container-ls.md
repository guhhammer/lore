# docker container ls

Muestra una lista de contenedores de Docker.

## Usage
- `docker ps` : muestra los contenedores de Docker que se están ejecutando actualmente
- `docker ps -a` : muestra todos los contenedores Docker (en ejecución y detenidos)
- `docker ps -l` : muestra el contenedor creado más recientemente (incluye todos los estados)
- `docker ps -f "name=nombre"` : filtra contenedores que contienen una subcadena en su nombre
- `docker ps -f "ancestor=image:tag"` : filtra contenedores que compartan una imagen determinada como antecesora
- `docker ps -f "exited=codigo" -a` : filtra contenedores por código de estado de salida
- `docker ps -f "status=estado"` : filtra contenedores por estado (creado, en ejecución, en eliminación, en pausa, cerrado o inactivo)
- `docker ps -f "volume=ruta/al/directorio" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : filtra contenedores que montan un volumen específico o tienen un volumen montado en una ruta específica

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
