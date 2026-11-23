# incus

Contenedor de sistemas y gestor de máquinas virtuales moderno, seguro y potente.

## Usage
- `incus list` : lista todos los contenedores y máquinas virtuales (tanto en ejecución como detenidas)
- `incus create imagen nombre_del_contenedor` : crea un contenedor a partir de una imagen, con un nombre personalizado
- `incus start|stop nombre_del_contenedor` : inicia o detiene un contenedor existente
- `incus shell nombre_del_contenedor` : abre un intérprete de comandos dentro de un contenedor en ejecución
- `incus delete nombre_del_contenedor` : elimina un contenedor detenido
- `incus copy remoto:imagen local:nombre_de_imagen_personalizada` : extrae una imagen de un repositorio de imágenes (remoto) al local
- `incus image list images:` : lista todas las imágenes disponibles en el repositorio oficial `images:` remoto
- `incus image list local:` : lista todas las imágenes ya descargadas en el remoto `local:`

## Links
- More information: https://linuxcontainers.org/incus/docs/main/
