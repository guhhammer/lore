# gzip

Comprime o descomprime archivos con la compresión `gzip` (LZ77).

## Usage
- `gzip ruta/al/archivo` : comprime un archivo, sustituyéndolo por un archivo `gzip`
- `gzip -d ruta/al/archivo.gz` : descomprime un archivo, sustituyéndolo por la versión original sin comprimir
- `gzip -v ruta/al/archivo` : muestra el nombre y el porcentaje de reducción de cada archivo comprimido
- `gzip -k ruta/al/archivo` : comprime un archivo, conservando el archivo original
- `gzip -c ruta/al/archivo >ruta/al/archivo_comprimido.gz` : comprime un archivo, especificando el nombre del archivo de salida
- `gzip -cd ruta/al/archivo.gz >ruta/al/archivo_descomprimido` : descomprime un archivo `gzip` especificando el nombre del archivo de salida
- `gzip -1..9 -c ruta/al/archivo >ruta/al/archivo_comprimido.gz` : especificando el nivel de compresión. 1 es el más rápido (compresión baja), 9 es el más lento (compresión alta) y 6 es el valor predeterminado
- `gzip -l ruta/al/archivo.txt.gz` : muestra el contenido de un archivo comprimido

## Links
- More information: https://www.gnu.org/software/gzip/manual/gzip.html
