# exa

Un reemplazo moderno para `ls` (Lista el contenido de los directorios).

## Usage
- `exa -1` : lista los archivos uno por línea
- `exa -a` : lista todos los archivos, incluidos los ocultos
- `exa -l -a` : lista en formato largo (permisos, propiedad, tamaño y fecha de modificación) de todos los archivos
- `exa -r -s size` : lista los archivos con el más grande en la parte superior
- `exa -l -T -L 3` : muestra un árbol de archivos, con tres niveles de profundidad
- `exa -l -s modified` : lista archivos ordenados por fecha de modificación (los más antiguos primero)
- `exa -l -h --icons --git` : muestra los archivos con sus encabezados, íconos y estados Git
- `exa --git-ignore` : no muestra los archivos mencionados en `.gitignore`

## Links
- More information: https://github.com/ogham/exa#command-line-options
