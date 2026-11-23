# partclone

Copia y restaura particiones desde y hacia una imagen sin tener en cuenta los bloques vacíos.

## Usage
- `sudo partclone.ext4|btrfs|fat32|xfs|... -c -s /dev/sdXY -o ruta/a/copia.img` : copia una partición en una imagen
- `sudo partclone.ext4|btrfs|fat32|xfs|... -c -s ruta/a/copia.img -o /dev/sdXY` : restaura una partición desde una imagen
- `partclone.ext4|btrfs|fat32|xfs|... -h` : muestra la ayuda

## Links
- More information: https://manned.org/partclone
