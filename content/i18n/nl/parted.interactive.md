# parted.interactive

Een programma voor het manipuleren van partities.

## Usage
- `sudo parted /dev/sdX` : start interactieve modus met de gespecificeerde schijf geselecteerd
- `print` : [Interactief] Toon partitie-informatie in interactieve modus
- `select /dev/sdX` : [Interactief] Selecteer een schijf in interactieve modus
- `mkpart partitie_naam btrfs|ext2|ext3|ext4|fat16|fat32|hfs|hfs+|linux-swap|ntfs|reiserfs|udf|xfs 0% 16G` : [Interactief] Maak een partitie van 16 GB aan met het gespecificeerde bestandssysteem in interactieve modus (`GPT`-partitietabel)
- `mkpart primary|logical|extended btrfs|ext2|ext3|ext4|fat16|fat32|hfs|hfs+|linux-swap|ntfs|reiserfs|udf|xfs 0% 16G` : [Interactief] Maak een partitie van 16 GB aan met het gespecificeerde bestandssysteem in interactieve modus (`MBR`-partitietabel)
- `resizepart /dev/sdXN eindpositie_van_partitie` : [Interactief] Wijzig de grootte van een partitie in interactieve modus
- `rm /dev/sdXN` : [Interactief] Verwijder een partitie in interactieve modus
- `?` : [Interactief] Toon de help

## Links
- More information: https://www.gnu.org/software/parted/parted.html
