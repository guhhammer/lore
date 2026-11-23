# mv

Mueve o renombra archivos y directorios.

## Usage
- `mv ruta/a/origen ruta/a/destino` : cambia el nombre de un archivo o directorio cuando el destino no es un directorio existente
- `mv ruta/a/origen ruta/al/directorio_existente` : mueve un archivo o directorio a un directorio existente
- `mv ruta/a/origen1 ruta/a/origen2 ... ruta/al/directorio_existente` : mueve varios archivos a un directorio existente, manteniendo los nombres de archivo sin cambios
- `mv -f ruta/a/origen ruta/a/destino` : no pedir confirmación antes de sobrescribir los archivos existentes
- `mv -i ruta/a/origen ruta/a/destino` : pide confirmación interactivamente antes de sobrescribir archivos existentes, independientemente de los permisos de los archivos
- `mv -n ruta/a/origen ruta/a/destino` : no sobrescribir los archivos existentes en el destino
- `mv -v ruta/a/origen ruta/a/destino` : mueve archivos en modo verboso, mostrando los archivos después de moverlos
- `find /var/log -type f -name '*.log' -print0 | xargs -0 mv -t ruta/al/directorio_destino` : especifica el directorio de destino para poder utilizar herramientas externas para recopilar archivos movibles

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/mv-invocation.html
