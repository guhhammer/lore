# apt-get

Utilidad de gestión de paquetes para Debian y Ubuntu. Búsqueda de paquetes mediante `apt-cache`.

## Usage
- `sudo apt-get update` : actualiza la lista de paquetes y versiones disponibles (se recomienda ejecutar esto antes de otros comandos `apt-get`)
- `sudo apt-get install paquete` : instala un paquete o lo actualiza a la última versión disponible
- `sudo apt-get remove paquete` : elimina un paquete
- `sudo apt-get purge paquete` : elimina un paquete y sus archivos de configuración
- `sudo apt-get upgrade` : actualiza todos los paquetes instalados a sus versiones más recientes
- `sudo apt-get autoclean` : limpia el repositorio local: elimina los archivos de paquetes (`.deb`) de descargas interrumpidas que ya no pueden descargarse
- `sudo apt-get autoremove` : elimina todos los paquetes que ya no sean necesarios
- `sudo apt-get dist-upgrade` : actualiza los paquetes instalados (como `upgrade`), pero eliminando los paquetes obsoletos e instalando paquetes adicionales para satisfacer las nuevas dependencias

## Links
- More information: https://manned.org/apt-get.8
