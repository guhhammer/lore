# od

Muestra el contenido del archivo en formato octal, decimal o hexadecimal. Opcionalmente, muestra las compensaciones de bytes y/o la representación imprimible de cada línea.

## Usage
- `od ruta/al/archivo` : muestra el archivo con la configuración predeterminada: formato octal, 8 bytes por línea, desplazamientos de bytes en octal y líneas duplicadas sustituidas por `*`
- `od -v ruta/al/archivo` : muestra el archivo en modo detallado, es decir, sin sustituir las líneas duplicadas por `*`
- `od -t x -A d -v ruta/al/archivo` : muestra el archivo en formato hexadecimal (unidades de 2 bytes), con desplazamientos de bytes en formato decimal
- `od -t x1 -w4 -v ruta/al/archivo` : muestra el archivo en formato hexadecimal (unidades de 1 byte) y 4 bytes por línea
- `od -t xz -A n -v ruta/al/archivo` : muestra el archivo en formato hexadecimal junto con su representación en caracteres y no imprime las compensaciones de bytes
- `od -N 100 -j 500 -v ruta/al/archivo` : lee solo 100 bytes de un archivo a partir del byte 500

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/od-invocation.html
