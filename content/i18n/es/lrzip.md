# lrzip

Un programa para comprimir archivos de gran tamaño.

## Usage
- `lrzip ruta/al/archivo` : comprime un archivo con LZMA: compresión lenta, descompresión rápida
- `lrzip -b ruta/al/archivo` : comprime un archivo con BZIP2: buen equilibrio entre compresión y velocidad
- `lrzip -z ruta/al/archivo` : comprime con ZPAQ: compresión extrema, pero muy lenta
- `lrzip -l ruta/al/archivo` : comprime con LZO: compresión ligera, descompresión extremadamente rápida
- `lrzip -e ruta/al/archivo` : comprime un archivo y lo protege con contraseña o lo cifra
- `lrzip -p 8 ruta/al/archivo` : especifica el número de subprocesos del procesador que se van a utilizar

## Links
- More information: https://manned.org/lrzip
