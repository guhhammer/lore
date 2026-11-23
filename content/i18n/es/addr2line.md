# addr2line

Convierte direcciones de un binario en nombres de archivos y números de línea.

## Usage
- `addr2line -e ruta/a/ejecutable address` : muestra el nombre de archivo y el número de línea del código fuente desde una dirección de instrucción de un ejecutable
- `addr2line -e ruta/a/ejecutable -f address` : muestra el nombre de la función, nombre de archivo y número de línea
- `addr2line -e ruta/a/ejecutable -f -C address` : decodifica (demangle) el nombre de la función para código C++

## Links
- More information: https://manned.org/addr2line
