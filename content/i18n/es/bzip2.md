# bzip2

Un compresor de archivos de clasificación por bloques.

## Usage
- `bzip2 ruta/al/archivo_a_comprimir` : comprime un archivo
- `bzip2 -d ruta/al/archivo_comprimido.bz2` : descomprime un archivo
- `bzip2 -dc ruta/al/archivo_comprimido.bz2` : descomprime un archivo en `stdout`
- `bzip2 -t ruta/al/archivo_comprimido.bz2` : comprueba la integridad de cada fichero dentro del archivo comprimido
- `bzip2 -v ruta/al/archivo_comprimido.bz2` : muestra la relación de compresión de cada archivo procesado con información detallada
- `bzip2 -f ruta/al/archivo_comprimido.bz2` : descomprime un archivo sobrescribiendo los archivos existentes
- `bzip2 -h` : muestra la ayuda

## Links
- More information: https://manned.org/bzip2
