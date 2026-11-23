# mkfs.cramfs

Crée un système de fichiers ROM à l’intérieur d’une partition.

## Usage
- `sudo mkfs.cramfs chemin/vers/dossier /dev/sdXY` : crée un système de fichiers ROM à partir d’un dossier dans la partition Y sur le périphérique X
- `sudo mkfs.cramfs -n nom_volume chemin/vers/dossier /dev/sdXY` : crée un système de fichiers ROM avec un nom de volume

## Links
- More information: https://manned.org/mkfs.cramfs
