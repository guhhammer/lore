# btrfs rescue

Intenta recuperar un sistema de archivos btrfs dañado.

## Usage
- `sudo btrfs resc ch ruta/a/la_partición` : reconstruye el árbol de metadatos del sistema de archivos (muy lento)
- `sudo btrfs resc fix-de ruta/a/la_partición` : corrige problemas relacionados con la alineación del tamaño del dispositivo (por ejemplo, incapacidad para montar el sistema de archivos debido a una discrepancia en los bytes totales del superbloque)
- `sudo btrfs resc s ruta/a/la_partición` : recupera un superbloque dañado de copias correctas (recupera la raíz del árbol de archivos del sistema)
- `sudo btrfs resc z ruta/a/la_partición` : recupera de transacciones interrumpidas (corrige problemas de reproducción de registros)
- `sudo btrfs resc c` : crea un dispositivo de control `/dev/btrfs-control` cuando `mknod` no está instalado

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-rescue.html
