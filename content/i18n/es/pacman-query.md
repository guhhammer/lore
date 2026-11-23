# pacman-query

Consulta la base de datos local de paquetes.

## Usage
- `pacman -Q` : consulta la base de datos local de paquetes y muestra los paquetes instalados y sus versiones
- `pacman -Qe` : muestra solo los paquetes y versiones que se instalaron explicitamente
- `pacman -Qo nombre_archivo` : averigua qué paquete gestiona un archivo
- `pacman -Qi paquete` : muestra información sobre un paquete instalado
- `pacman -Ql paquete` : muestra la lista de archivos que pertenecen a un paquete específico
- `pacman -Qdtq` : muestra los paquetes huérfanos (instalados como dependencias pero que ningún paquete (t) requiere, y los muestra en modo quieto (solo se muestra el nombre del paquete))
- `pacman -Qm` : muestra los paquetes instalados ajenos (m) a la base de datos del repositorio
- `pacman -Qu` : muestra los paquetes que se pueden actualizar

## Links
- More information: https://manned.org/pacman.8
