# pacman-sync

Utilidad del administrador de paquetes de Arch Linux.

## Usage
- `sudo pacman -S paquete` : instala un paquete nuevo
- `sudo pacman -Syu` : sincroniza y actualiza (y) la base de datos de paquetes junto con un sysupgrade (añade `--downloadonly` para solamente descargar los paquetes y no actualizarlos)
- `sudo pacman -Syu --noconfirm paquete` : actualiza (update) y moderniza (upgrade) todos los paquetes e instala uno nuevo sin solicitar confirmación
- `pacman -Ss "patrón_de_búsqueda"` : busca (s) la base de datos de paquetes con una `regex` o palabra clave
- `pacman -Si paquete` : muestra información sobre un paquete
- `sudo pacman -Syu --overwrite ruta/al/archivo` : sobrescribe archivos conflictivos durante una actualización del paquete
- `sudo pacman -Syu --ignore paquete1 paquete2 ...` : sincroniza y actualiza todos los paquetes, e ignora un paquete específico (puede ser utilizado más de una vez)
- `sudo pacman -Sc` : elimina paquetes no instalados y repositorios no utilizados de la caché (utiliza las banderas `Sc` para limpiar (c) todos los paquetes)

## Links
- More information: https://manned.org/pacman.8
