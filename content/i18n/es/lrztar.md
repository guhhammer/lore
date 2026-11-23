# lrztar

Una herramienta de encapsulado para `lrzip` que simplifica la compresión de directorios.

## Usage
- `lrztar ruta/al/directorio` : archiva un directorio con `tar` y, a continuación, comprimirlo
- `lrztar -z ruta/al/directorio` : igual que arriba, pero con ZPAQ (compresión extrema, aunque muy lenta)
- `lrztar -o ruta/al/archivo ruta/al/directorio` : especifica el archivo de salida
- `lrztar -p 8 ruta/al/directorio` : anula el número de subprocesos del procesador que se van a utilizar
- `lrztar -f ruta/al/directorio` : obliga la sobrescritura de los archivos existentes

## Links
- More information: https://manned.org/lrztar
