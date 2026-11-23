# dnf

Gestor de paquetes para Fedora 41+ y RHEL 10.

## Usage
- `sudo dnf up` : actualiza los paquetes instalados a las últimas versiones disponibles
- `dnf se keyword1 keyword2 ...` : busca paquetes por palabras clave
- `dnf if paquete` : muestra detalles sobre un paquete
- `sudo dnf in paquete1 paquete2 ...` : instala nuevos paquetes (utilice `--assumeyes` para confirmar automáticamente todas las solicitudes)
- `sudo dnf rm paquete1 paquete2 ...` : elimina paquetes
- `dnf ls --installed` : muestra una lista de los paquetes instalados
- `dnf provides comando` : busca qué paquetes proporcionan un comando determinado
- `sudo dnf clean all|dbcache|expire-cache|metadata|packages` : limpia los datos almacenados en caché

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
