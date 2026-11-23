# mktemp

Crea un archivo o directorio temporal.

## Usage
- `mktemp` : crea un archivo temporal vacío e imprime su ruta absoluta
- `mktemp --tmpdir /ruta/a/tempdir` : usa un directorio personalizado (por defecto la salida de `getconf DARWIN_USER_TEMP_DIR`, o `/tmp`)
- `mktemp /tmp/ejemplo.XXXXXXXX` : usa una plantilla de ruta personalizada (las `X` se sustituyen por caracteres alfanuméricos aleatorios)
- `mktemp -t ejemplo` : usa un prefijo de nombre de archivo personalizado
- `mktemp --directory` : crea un directorio temporal vacío e imprime su ruta absoluta

## Links
- More information: https://keith.github.io/xcode-man-pages/mktemp.1.html
