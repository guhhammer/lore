# findmnt

Busca tu sistema de archivos.

## Usage
- `findmnt` : lista todos los sistemas de archivos montados
- `findmnt /dev/sdb1` : busca un dispositivo
- `findmnt /` : busca un punto de montaje
- `findmnt -t ext4,virtiofs,btrfs,...` : busca sistemas de archivos de un tipo específico
- `findmnt LABEL=BigStorage` : busca sistemas de archivos con una etiqueta específica
- `findmnt -x --verbose` : comprueba en detalle el contenido de la tabla de montaje y verifica `/etc/fstab`

## Links
- More information: https://manned.org/findmnt
