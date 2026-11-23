# apt-clone

Clona/hace copia de seguridad/restaura el estado de un paquete de un sistema basado en Debian.

## Usage
- `apt-clone clone ruta/al/directorio` : clona el estado del paquete del sistema actual en un directorio especificado
- `apt-clone clone --destination ruta/a/backup.tar.gz` : crea un archivo clon (`.tar.gz`) con fines de copia de seguridad
- `apt-clone restore ruta/a/backup.tar.gz` : restaura el estado del paquete desde un archivo clon
- `apt-clone info ruta/a/backup.tar.gz` : muestra información sobre un archivo clon (por ejemplo, la versión, la arquitectura)
- `apt-clone restore ruta/a/backup.tar.gz --destination ruta/a/restaurar` : comprueba si el archivo clon se puede restaurar en el sistema actual

## Links
- More information: https://github.com/mvo5/apt-clone
