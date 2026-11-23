# bzip3

Un eficiente compresor estadístico de archivos.

## Usage
- `bzip3 ruta/al/archivo_a_comprimir` : comprime un archivo
- `bzip3 -d ruta/al/archivo_comprimido.bz3` : descomprime un archivo
- `bzip3 -dc ruta/al/archivo_comprimido.bz3` : descomprime un archivo en `stdout`
- `bzip3 -t ruta/al/archivo_comprimido.bz3` : comprueba la integridad de cada fichero dentro del archivo comprimido
- `bzip3 -v ruta/al/archivo_comprimido.bz3` : muestra la relación de compresión de cada archivo procesado con información detallada
- `bzip3 -d -f ruta/al/archivo_comprimido.bz3` : descomprime un archivo sobrescribiendo los archivos existentes
- `bzip3 -h` : muestra la ayuda

## Links
- More information: https://manned.org/bzip3
