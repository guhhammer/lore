# pkgfile

Busca archivos en los paquetes de los repositorios oficiales en sistemas basados en Arch.

## Usage
- `sudo pkgfile --update` : sincroniza la base de datos pkgfile
- `pkgfile archivo` : busca un paquete que posee un archivo específico
- `pkgfile --list paquete` : lista todos los archivos proporcionados por un paquete
- `pkgfile --list --binaries paquete` : lista los ejecutables proporcionados por un paquete
- `pkgfile --ignorecase archivo` : busca un paquete que posee un archivo específico utilizando coincidencias insensibles a mayúsculas y minúsculas
- `pkgfile --binaries archivo` : busca un paquete que posee un archivo específico en el directorio `bin` o `sbin`
- `pkgfile --verbose archivo` : busca un paquete que posee un archivo específico, mostrando la versión del paquete
- `pkgfile --repo nombre_del_repositorio archivo` : busca un paquete que posee un archivo específico en un repositorio específico

## Links
- More information: https://manned.org/pkgfile
