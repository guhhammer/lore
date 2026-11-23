# tree

Muestra los contenidos del directorio actual en forma de árbol.

## Usage
- `tree -L num` : imprime archivos y directorios hasta `num` niveles de profundidad (donde 1 significa el directorio actual)
- `tree -d` : imprime solo directorios
- `tree -a -C` : imprime también archivos ocultos, coloreando la salida
- `tree -i -f` : imprime el árbol sin sangría, mostrando la ruta completa en su lugar (use `-N` para evitar escapar caracteres no imprimibles)
- `tree -s -h --du` : imprime el tamaño de cada archivo y el tamaño total de cada directorio en formato legible para humanos
- `tree -P '*.txt' --prune` : imprime archivos dentro de la jerarquía de árbol, especificando un patrón comodín, excluyendo los directorios que no contengan archivos coincidentes
- `tree -P nombre_del_directorio --matchdirs --prune` : imprime archivos dentro de la jerarquía de árbol, especificando un patrón, excluyendo los directorios que no sean ancestros del especificado
- `tree -I 'nombre_del_directorio1|nombre_del_directorio2'` : imprime el árbol ignorando los directorios especificados

## Links
- More information: https://manned.org/tree
