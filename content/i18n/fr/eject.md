# eject

Éjecte les CD, disquettes et les bandes magnétiques.

## Usage
- `eject -d` : affiche l'appareil par défaut
- `eject` : éjecte l'appareil par défaut
- `eject /dev/cdrom` : éjecte un appareil spécifique (l'ordre par défaut est CD-ROM, SCSI, Disquette puis bande magnétique)
- `eject -T /dev/cdrom` : bascule le support d'appareil en mode ouvert ou fermé
- `eject -r /dev/cdrom` : éjecte un CD
- `eject -f /mnt/disquette` : éjecte une disquette
- `eject -q /mnt/bande` : éjecte une bande magnétique

## Links
- More information: https://manned.org/eject
