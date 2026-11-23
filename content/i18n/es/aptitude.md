# aptitude

Herramienta de gestión de paquetes para Debian y Ubuntu.

## Usage
- `sudo aptitude update` : sincroniza la lista de paquetes y versiones disponible (se recomienda ejecutar este comando antes que cualquier otro comando `aptitude`)
- `sudo aptitude install paquete` : instala un nuevo paquete y sus dependencias
- `aptitude search paquete` : busca un paquete
- `aptitude search '?installed(paquete)'` : busca un paquete instalado (`?installed` es un término de búsqueda de `aptitude`)
- `sudo aptitude remove paquete` : elimina un paquete y todos los paquetes que dependen de él
- `sudo aptitude upgrade` : actualiza todos los paquetes a sus nuevas versiones disponibles
- `sudo aptitude full-upgrade` : actualiza paquetes instalados (como `aptitude upgrade`), elimina los paquetes obsoletos e instala paquetes adicionales para satisfacer sus dependencias
- `sudo aptitude hold '?installed(paquete)'` : mantiene un paquete instalado para que no sea actualizado automáticamente

## Links
- More information: https://manned.org/aptitude
