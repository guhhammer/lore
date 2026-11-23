# docker container exec

Ejecuta un comando en un contenedor Docker que ya se está ejecutando.

## Usage
- `docker exec -it nombre_contenedor /bin/bash` : abre una sesión de intérprete de comandos interactiva en un contenedor que ya está funcionando
- `docker exec -d nombre_contenedor comando` : ejecuta un comando en segundo plano (desconectado) en un contenedor en ejecución
- `docker exec -it -w ruta/al/directorio nombre_del_contenedor comando` : selecciona el directorio de trabajo para ejecutar un comando determinado
- `docker exec -i -d nombre_contenedor comando` : ejecuta un comando en segundo plano en un contenedor existente, pero mantener `stdin` abierto
- `docker exec -it -e nombre_variable=valor nombre_contenedor /bin/bash` : establece una variable de entorno en una sesión Bash en ejecución
- `docker exec -u usuario nombre_contenedor comando` : ejecuta un comando como un usuario específico

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
