# hexdump

Un visor ASCII, decimal, hexadecimal, octal.

## Usage
- `hexdump ruta/al/archivo` : imprime la representación hexadecimal de un archivo, reemplazando líneas duplicadas con '*'
- `hexdump -C ruta/al/archivo` : muestra el desplazamiento de entrada (input offset) en hexadecimal y su representación ASCII en dos columnas
- `hexdump -C -n número_de_bytes ruta/al/archivo` : muestra la representación hexadecimal de un archivo, pero interpreta solo n bytes de la entrada
- `hexdump -v ruta/al/archivo` : no reemplaza las líneas duplicadas con '*'

## Links
- More information: https://manned.org/hexdump
