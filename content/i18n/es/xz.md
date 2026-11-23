# xz

Comprime o descomprime archivos XZ y LZMA.

## Usage
- `xz ruta/al/archivo` : comprime un archivo usando xz
- `xz -d ruta/al/archivo.xz` : descomprime un archivo XZ
- `xz -F lzma ruta/al/archivo` : comprime un archivo usando LZMA
- `xz -d -F lzma ruta/al/archivo.lzma` : descomprime un archivo LZMA
- `xz -d -c ruta/al/archivo.xz` : descomprime un archivo y escribe a `stdout` (implica `--keep`)
- `xz -k ruta/al/archivo` : comprime un archivo, pero no borra el original
- `xz -0 ruta/al/archivo` : comprime un archivo con la compresión más rápida
- `xz -9 ruta/al/archivo` : comprime un archivo con la mejor compresión

## Links
- More information: https://manned.org/xz
