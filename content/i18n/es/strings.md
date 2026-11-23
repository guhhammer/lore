# strings

Encuentra cadenas imprimibles en un archivo objeto o binario.

## Usage
- `strings ruta/al/archivo` : imprime todas las cadenas contenidas en un binario
- `strings -n n ruta/al/archivo` : limita resultados a cadenas por lo menos n caracteres de largo
- `strings -t d ruta/al/archivo` : precede cada resultado con su distancia hasta el inicio (offset) del archivo
- `strings -t x ruta/al/archivo` : precede cada resultado con su desplazamiento (offset) dentro del archivo en hexadecimal

## Links
- More information: https://manned.org/strings
