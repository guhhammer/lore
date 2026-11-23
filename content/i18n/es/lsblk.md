# lsblk

Lista información sobre dispositivos.

## Usage
- `lsblk` : lista todos los dispositivos de almacenamiento en formato de árbol
- `lsblk -a` : lista también los dispositivos vacíos
- `lsblk -b` : muestra la columna SIZE en bytes en lugar de un formato legible por humanos
- `lsblk -f` : muestra información sobre los sistemas de archivos
- `lsblk -i` : usa caracteres ASCII para dibujar el árbol
- `lsblk -t` : muestra información sobre la topología de los dispositivos de bloque
- `lsblk -e 1,7,...` : excluye los dispositivos indicados mediante una lista de números mayores de dispositivo separados por comas
- `lsblk -o +NAME,ROTA,SERIAL,MODEL,TRAN,TYPE,SIZE,FSTYPE,MOUNTPOINT,...` : añade información extra a la salida usando una lista de columnas separadas por comas (omite el signo `+` para mostrar solo las columnas indicadas)

## Links
- More information: https://manned.org/lsblk
