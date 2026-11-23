# nm

Lista los nombres de símbolos en los archivos objeto.

## Usage
- `nm -g ruta/al/archivo.o` : lista de funciones globales (externas) en un archivo (con prefijo T)
- `nm -u ruta/al/archivo.o` : lista solo los símbolos no definidos en un archivo
- `nm -a ruta/al/archivo.o` : lista todos los símbolos, incluso símbolos de depuración
- `nm -C ruta/al/archivo.o` : reconstruye símbolos C++ (hace que sean legibles)

## Links
- More information: https://manned.org/nm
