# split

Divide un archivo en trozos.

## Usage
- `split -l 10 ruta/al/archivo` : divide un archivo, cada división tiene 10 líneas (excepto la última división)
- `split -p cat|^[dh]og ruta/al/archivo` : divide un fichero mediante una `regex`. La línea que coincida será la primera línea del siguiente archivo de salida
- `split -b 512 ruta/al/archivo` : divide un archivo con 512 bytes en cada división (excepto en la última; utiliza 512k para kilobytes y 512m para megabytes)
- `split -n 5 ruta/al/archivo` : divide un archivo en 5 archivos. El archivo se divide de forma que cada división tenga el mismo tamaño (excepto la última división)

## Links
- More information: https://keith.github.io/xcode-man-pages/split.1.html
