# cat

Imprime y concatena archivos.

## Usage
- `cat ruta/al/archivo` : imprime el contenido de un fichero a `stdout`
- `cat ruta/al/archivo1 ruta/al/archivo2 ... >ruta/al/archivo_salida` : concatena varios archivos en un archivo de salida
- `cat ruta/al/archivo1 ruta/al/archivo2 ... >>ruta/al/archivo_salida` : añade el contenido de varios archivos a un archivo de salida
- `cat -u /dev/tty12 >/dev/tty13` : copia el contenido de un archivo en un archivo de salida sin almacenamiento en el búfer
- `cat - >ruta/al/archivo` : copia `stdin` en un archivo

## Links
- More information: https://manned.org/cat.1posix
