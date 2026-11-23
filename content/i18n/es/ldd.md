# ldd

Muestra dependencias de bibliotecas compartidas de un binario. No use en un binario no confiable, use `objdump` para esto en su lugar.

## Usage
- `ldd ruta/al/binario` : muestra dependencias de biblioteca compartidas de un binario
- `ldd -v ruta/al/binario` : muestra toda la información sobre las dependencias
- `ldd -u ruta/al/binario` : muestra dependencias directas no utilizadas
- `ldd -d ruta/al/binario` : reporta objetos de datos perdidos y realiza reubicaciones de datos
- `ldd -r ruta/al/binario` : reporta objetos y funciones de datos ausentes y los reubica a ambos

## Links
- More information: https://manned.org/ldd
