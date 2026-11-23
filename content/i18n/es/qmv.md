# qmv

Mueve archivos y directorios usando el editor de texto predeterminado para definir los nombres de archivos.

## Usage
- `qmv archivo_original` : mueve un solo archivo (abre un editor con el nombre de archivo fuente a la izquierda y el nombre de archivo de destino a la derecha)
- `qmv *.jpg` : mueve múltiples archivos JPEG
- `qmv -d ruta/al/directorio1 ruta/al/directorio2 ruta/al/directorio3 ...` : mueve múltiples directorios
- `qmv -R ruta/al/directorio` : mueve todos los archivos y directorios dentro de un directorio
- `qmv -o swap *.jpg` : mueve archivos, pero cambia las posiciones de la fuente y los nombres de archivo de destino en el editor
- `qmv -f do .` : renombra todos los archivos y carpetas en el directorio actual, pero muestra solo los nombres de archivo de destino en el editor (puedes pensar en ello como una especie de modo simple)

## Links
- More information: https://manned.org/qmv
