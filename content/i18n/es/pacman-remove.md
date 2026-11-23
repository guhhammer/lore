# pacman-remove

Utilidad del administrador de paquetes de Arch Linux.

## Usage
- `sudo pacman -Rs paquete` : remueve un paquete y sus dependencias recursivamente
- `sudo pacman -Rsn paquete` : remueve un paquete y sus dependencias. no guarda copias de seguridad de los archivos de configuración
- `sudo pacman -R --noconfirm paquete` : remueve un paquete sin pedir confirmación
- `sudo pacman -Rsn $(pacman -Qdtq)` : remueve los paquetes huérfanos (instalados como dependencias pero no requeridos por algún paquete)
- `sudo pacman -Rc paquete` : remueve un paquete y hace lo mismo a todos los paquetes que dependen de él
- `pacman -Rp paquete` : lista e imprime los paquetes que serían afectados (no Remueve paquete alguno)
- `pacman -Rh` : muestra la ayuda

## Links
- More information: https://manned.org/pacman.8
