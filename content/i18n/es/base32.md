# base32

Codifica o decodifica un archivo o la entrada estandar hacia/desde Base32, a la salida estandar.

## Usage
- `base32 ruta/al/archivo` : codifica un archivo
- `base32 -w 0|76|... ruta/al/archivo` : ajuste la salida codificada en un ancho específico (`0` deshabilita el ajuste)
- `base32 -d ruta/al/archivo` : decodifica un archivo
- `comando | base32` : codifica `stdin`
- `comando | base32 -d` : decodifica `stdin`

## Links
- More information: https://manned.org/base32
