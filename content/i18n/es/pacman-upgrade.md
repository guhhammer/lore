# pacman-upgrade

Instala paquetes manualmente a partir de archivos comprimidos.

## Usage
- `sudo pacman -U ruta/al/paquete1.pkg.tar.zst ruta/al/paquete2.pkg.tar.zst ...` : instala uno o más paquetes desde archivos
- `sudo pacman -U --noconfirm ruta/al/paquete.pkg.tar.zst` : instala un paquete sin solicitar confirmación
- `sudo pacman -U --overwrite ruta/al/archivo ruta/al/paquete.pkg.tar.zst` : sobrescribe archivos conflictivos durante la instalación de un paquete
- `sudo pacman -Ud ruta/al/paquete.pkg.tar.zst` : instala un paquete, omitiendo las comprobaciones de versión de las dependencias
- `pacman -Up ruta/al/paquete.pkg.tar.zst` : obtiene e imprime los paquetes que se verían afectados por la actualización (no instala ningún paquete)
- `pacman -Uh` : muestra la ayuda

## Links
- More information: https://manned.org/pacman.8
