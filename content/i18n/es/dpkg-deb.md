# dpkg-deb

Empaqueta, desempaqueta y proporciona información sobre archivos Debian.

## Usage
- `dpkg-deb -I ruta/al/archivo.deb` : muestra información sobre un paquete
- `dpkg-deb -W ruta/al/archivo.deb` : muestra el nombre y la versión del paquete en una línea
- `dpkg-deb -c ruta/al/archivo.deb` : lista el contenido del paquete
- `dpkg-deb -x ruta/al/archivo.deb ruta/al/directorio` : extrae el contenido del paquete en un directorio
- `dpkg-deb -b ruta/al/directorio` : crea un paquete desde un directorio especificado

## Links
- More information: https://manned.org/dpkg-deb
