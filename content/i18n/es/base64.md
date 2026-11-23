# base64

Codifica o decodifica un archivo o la entrada estandar hacia/desde Base64, a la salida estandar.

## Usage
- `base64 ruta/al/archivo` : codifica un archivo
- `base64 -w 0|76|... ruta/al/archivo` : ajusta la salida codificada en un ancho específico (`0` deshabilita el ajuste)
- `base64 -d ruta/al/archivo` : decodifica un archivo
- `comando | base64` : codifica `stdin`
- `comando | base64 -d` : decodifica `stdin`

## Links
- More information: https://manned.org/base64
