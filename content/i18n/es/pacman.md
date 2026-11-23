# pacman

Arch Linux paquete manager utility. Utilidad del administrador de paquetes de Arch Linux.

## Usage
- `sudo pacman -Syu` : sincroniza y actualiza todos los paquetes
- `sudo pacman -S paquete` : instala un nuevo paquete
- `sudo pacman -Rs paquete` : elimina un paquete y sus dependencias
- `pacman -F "nombre_del_archivo"` : busca en la base de datos paquetes que contengan un archivo específico
- `pacman -Q` : lista los paquetes y versiones instalados
- `pacman -Qe` : lista solo los paquetes y versiones instalados explícitamente
- `pacman -Qtdq` : lista los paquetes huérfanos (instalados como dependencias pero que en realidad no son necesarios para ningún paquete)
- `sudo pacman -Scc` : vacía toda la caché de `pacman`

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
