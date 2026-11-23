# umount

Koppel een bestandssysteem los vanuit het mountpunt, waardoor het niet langer toegankelijk is. Een bestandssysteem kan niet losgekoppeld worden als het bezig is.

## Usage
- `umount pad/naar/apparaat_bestand` : koppel een bestandssysteem los door het pad op te geven van de bron waarop het gemount is
- `umount pad/naar/gemounte_map` : koppel een bestandssysteem los door het pad op te geven waarop het gemount is
- `umount -a` : koppel alle gemounte bestandssystemen los (behalve het `proc` bestandssysteem)

## Links
- More information: https://man.openbsd.org/umount
