# docker container update

Actualiza la configuración de los contenedores de Docker. Nota: Este comando no es compatible con contenedores de la plataforma Windows.

## Usage
- `docker update --restart always|no|on-failure|unless-stopped nombre_contenedor` : actualiza la política de reinicio para que se aplique cuando un contenedor específico se cierre
- `docker update --restart on-failure:3 nombre_contenedor` : actualiza la política para reiniciar hasta tres veces un contenedor específico cuando se cierre con un estado de salida distinto de cero
- `docker update --cpus count nombre_contenedor` : actualiza el número de CPU disponibles para un contenedor específico
- `docker update -m limitM nombre_contenedor` : actualiza el límite de memoria en Megabytes para un contenedor específico
- `docker update --pids-limit count nombre_contenedor` : actualiza el número máximo de ID de proceso permitidos dentro de un contenedor específico (utilice `-1` para ilimitado)
- `docker update --memory-swap limitM nombre_contenedor` : actualiza la cantidad de memoria en Megabytes que un contenedor específico puede intercambiar al disco (utilice `-1` para ilimitado)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/update/
