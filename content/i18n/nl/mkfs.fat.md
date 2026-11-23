# mkfs.fat

Maak een MS-DOS bestandssysteem in een partitie.

## Usage
- `sudo mkfs.fat /dev/sdXY` : maak een fat bestandssysteem binnen partitie `Y` op apparaat `X`
- `sudo mkfs.fat -n volume_naam /dev/sdXY` : maak een bestandssysteem met een volumenaam
- `sudo mkfs.fat -i volume_id /dev/sdXY` : maak een bestandssysteem met een volume-id
- `sudo mkfs.fat -f 4 /dev/sdXY` : gebruik 4 in plaats van 2 bestandstoewijzingstabellen
- `sudo mkfs.fat -F 12|16|32 /dev/sdXY` : geef bestandssysteemtype op

## Links
- More information: https://manned.org/mkfs.fat
