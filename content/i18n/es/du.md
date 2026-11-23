# du

Uso del disco: estima y resume el uso del espacio de archivos y directorios.

## Usage
- `du -b|k|m ruta/al/directorio` : lista los tamaños de un directorio y sus subdirectorios, en la unidad dada (B/KiB/MiB)
- `du -h ruta/al/directorio` : lista los tamaños de un directorio y sus subdirectorios, de forma legible (es decir, seleccionando automáticamente la unidad adecuada para cada tamaño)
- `du -sh ruta/al/directorio` : muestra el tamaño de un único directorio, en unidades legibles
- `du -ah ruta/al/directorio` : lista los tamaños legibles de un directorio y de todos los archivos y directorios que contiene
- `du -h --max-depth=N ruta/al/directorio` : lista los tamaños legibles de un directorio y sus subdirectorios, hasta N niveles de profundidad
- `du -ch */*.jpg` : lista el tamaño legible de todos los archivos `.jpg` en los subdirectorios del directorio actual y muestra un total acumulado al final
- `du -ah -t 1G|1024M|1048576K .[^.]* *` : lista todos los archivos y directorios (incluidos los ocultos) por encima de un determinado tamaño (útil para investigar qué está ocupando realmente el espacio)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/du-invocation.html
