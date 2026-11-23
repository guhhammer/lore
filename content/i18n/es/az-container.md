# az container

Administra instancias de Azure Container. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az container create -g grupo_de_recursos -n nombre --image nombre_de_la_imagen -os windows|linux --cpu número_de_núcleos_del_cpu --memory capacidad_de_memoria_en_GB` : crea un contenedor en un grupo de contenedores
- `az container exec -g grupo_de_recursos -n nombre_del_grupo_de_contenedores --exec-command "comando"` : ejecuta un comando desde un contenedor en ejecución dentro de un grupo de contenedores
- `az container logs -n nombre -g grupo_de_recursos` : examina los registros de un contenedor en un grupo de contenedores
- `az container show -n nombre -g grupo_de_recursos` : visualiza los detalles de un grupo de contenedores
- `az container start -n nombre -g grupo_de_recursos` : inicia todos los contenedores de un grupo de contenedores
- `az container stop -n nombre -g grupo_de_recursos` : detiene todos los contenedores de un grupo de contenedores
- `az container delete -n nombre -g grupo_de_recursos` : elimina un grupo de contenedores

## Links
- More information: https://learn.microsoft.com/cli/azure/container
