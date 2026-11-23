# debootstrap

Crea un sistema básico de Debian.

## Usage
- `sudo debootstrap stable ruta/a/debian-root/ http://deb.debian.org/debian` : crea un sistema de la versión estable de Debian dentro del directorio `debian-root`
- `sudo debootstrap --variant=minbase stable ruta/a/debian-root/` : crea un sistema mínimo que incluye solo los paquetes necesarios
- `sudo debootstrap focal ruta/a/focal-root/ file:///ruta/a/mirror/` : crea un sistema Ubuntu 20.04 dentro del directorio `focal-root` con un espejo local
- `sudo chroot ruta/a/root` : cambia a un sistema de arranque
- `ls /usr/share/debootstrap/scripts/` : lista las versiones mayores disponibles

## Links
- More information: https://wiki.debian.org/Debootstrap
