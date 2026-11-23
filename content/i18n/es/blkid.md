# blkid

Lista todas las particiones reconocidas y su identificador único universal (UUID).

## Usage
- `sudo blkid` : lista todas las particiones
- `sudo blkid -o list` : lista todas las particiones en una tabla, incluidos los puntos de montaje actuales
- `sudo blkid -s UUID -o value /dev/sdXY` : obtiene el UUID del sistema de archivos en una partición

## Links
- More information: https://manned.org/blkid
