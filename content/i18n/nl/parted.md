# parted

Een programma voor het manipuleren van partities.

## Usage
- `sudo parted -l` : toon partities op alle blokapparaten
- `sudo parted /dev/sdX mklabel aix|amiga|bsd|dvh|gpt|loop|mac|msdos|pc98|sun` : maak een nieuwe partitietabel van het gespecificeerde label-type
- `sudo parted /dev/sdX -s mklabel gpt mkpart "boot_partitie_naam" 0% 500MiB mkpart "systeem_partitie_naam" 500MiB 100%` : maak een nieuwe `gpt`-partitietabel met een boot-partitie van 500MiB en geef de rest aan de systeempartitie (`--script` slaat gebruikersinvoerverzoeken over)
- `sudo parted /dev/sdX set 1 boot on` : stel een partitie in met de boot-vlag ingeschakeld
- `sudo parted /dev/sdX` : start interactieve modus met de gespecificeerde schijf geselecteerd
- `parted -h` : toon de help

## Links
- More information: https://www.gnu.org/software/parted/manual/parted.html#Invoking-Parted
