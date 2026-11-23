# dpkg-query

Muestra información sobre paquetes instalados.

## Usage
- `dpkg-query -l` : lista todos los paquetes instalados
- `dpkg-query -l 'libc6*'` : lista de paquetes instalados que coinciden con un patrón
- `dpkg-query -L libc6` : lista todos los archivos instalados por un paquete
- `dpkg-query -s libc6` : muestra información sobre un paquete
- `dpkg-query -S /etc/ld.so.conf.d` : busca paquetes que tengan archivos que coincidan con un patrón

## Links
- More information: https://manned.org/dpkg-query
