# debtap

Convierte paquetes Debian en paquetes de Arch Linux.

## Usage
- `sudo debtap -u` : actualiza la base de datos de debtap (antes de la primera ejecución)
- `debtap ruta/al/paquete.deb` : convierte el paquete especificado
- `debtap -q ruta/al/paquete.deb` : convierte el paquete especificado obviando todas las preguntas, excepto para editar archivos de metadatos
- `debtap -p ruta/al/paquete.deb` : genera un archivo PKGBUILD

## Links
- More information: https://github.com/helixarch/debtap#available-options
