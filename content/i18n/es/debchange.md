# debchange

Mantiene el archivo debian/log de cambios (changelog) de un paquete fuente de Debian.

## Usage
- `debchange -n` : agrega una nueva versión para una subida que no es del mantenedor al registro (log) de cambios
- `debchange -a` : agrega una entrada de cambio a la versión actual
- `debchange --closes id_del_fallo` : agrega una entrada de cambio para cerrar el fallo con un ID específico

## Links
- More information: https://manned.org/debchange
