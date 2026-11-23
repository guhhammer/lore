# sfdisk

Toon of manipuleer een schijfpartitietabel.

## Usage
- `sudo sfdisk -d /dev/sdX >pad/naar/bestand.dump` : sla een backup van de partitie-indeling op naar een bestand
- `sudo sfdisk <pad/naar/bestand.dump /dev/sdX` : herstel een partitie-indeling
- `sudo sfdisk --part-type /dev/sdX partitie_nummer swap` : stel het partitietype in
- `sudo sfdisk --delete /dev/sdX partitie_nummer` : verwijder een partitie
- `sfdisk -h` : toon de help

## Links
- More information: https://manned.org/sfdisk
