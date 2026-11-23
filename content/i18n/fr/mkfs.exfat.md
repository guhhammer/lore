# mkfs.exfat

Crée un système de fichiers exfat à l’intérieur d’une partition.

## Usage
- `sudo mkfs.exfat /dev/sdXY` : crée un système de fichiers exfat dans la partition Y sur le périphérique X
- `sudo mkfs.exfat -L nom_volume /dev/sdXY` : crée un système de fichiers avec un nom de volume
- `sudo mkfs.exfat -U id_volume /dev/sdXY` : crée un système de fichiers avec un identifiant de volume

## Links
- More information: https://manned.org/mkfs.exfat
