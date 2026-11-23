# apt

Herramienta de gestión de paquete para distribuciones basadas en Debian. Se recomienda sustituirlo por `apt-get` cuando se use interactivamente en Ubuntu 16.04 o versiones posteriores.

## Usage
- `sudo apt update` : actualiza la lista de paquetes y versiones disponibles (se recomienda ejecutar este comando antes que cualquier otro comando `apt`)
- `apt search paquete` : busca un paquete
- `apt show paquete` : muestra la información de un paquete
- `sudo apt install paquete` : instala un paquete o lo actualiza a su última versión disponible
- `sudo apt remove paquete` : elimina un paquete (si se utiliza `purge` también elimina sus archivos de configuración)
- `sudo apt upgrade` : actualiza todos los paquetes a sus nuevas versiones disponibles
- `apt list` : muestra todos los paquetes
- `apt list -i` : muestra los paquetes instalados

## Links
- More information: https://manned.org/apt.8
