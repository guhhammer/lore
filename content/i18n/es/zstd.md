# zstd

Comprime o descomprime archivos con compresión Zstandard.

## Usage
- `zstd ruta/al/archivo` : comprime un archivo en un nuevo archivo con el sufijo `.zst`
- `zstd --decompress ruta/al/archivo.zst` : descomprime un archivo
- `zstd --decompress --stdout ruta/al/archivo.zst` : descomprime a la salida estándar `stdout`
- `zstd -nivel ruta/al/archivo` : comprime un archivo especificando el nivel de compresión, donde 1=más rápido, 19=más lento y 3=predeterminado
- `zstd --ultra -nivel ruta/al/archivo` : desbloquea niveles de compresión superiores (hasta 22) utilizando más memoria (tanto para compresión como descompresión)

## Links
- More information: https://manned.org/zstd
