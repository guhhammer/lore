# readelf

Muestra información sobre archivos ELF.

## Usage
- `readelf -all ruta/al/archivo_binario` : muestra toda la información de un archivo ELF
- `readelf --headers ruta/al/archivo_binario` : muestra todas las cabeceras presentes en un archivo ELF
- `readelf --symbols ruta/al/archivo_binario` : muestra las entradas en la sección de la tabla de símbolos del archivo ELF, si tiene una
- `readelf --file-header ruta/al/archivo_binario` : muestra la información de la cabecera ELF
- `readelf --section-headers ruta/al/archivo_binario` : muestra información de cabecera de la sección ELF

## Links
- More information: https://manned.org/readelf
