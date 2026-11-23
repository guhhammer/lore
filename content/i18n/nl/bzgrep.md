# bzgrep

Zoek patronen in `bzip2` gecomprimeerde bestanden met behulp van `grep`.

## Usage
- `bzgrep "zoekpatroon" pad/naar/bestand` : zoek naar een patroon in een gecomprimeerd bestand
- `bzgrep -r "zoekpatroon" pad/naar/tar_bestand` : zoek recursief naar een patroon in bestanden in een `bzip2` gecomprimeerd `.tar`-archief
- `bzgrep --context|--before-context|--after-context 3 "zoekpatroon" pad/naar/bestand` : toon 3 regels rondom Context, voor (B) of nA elke overeenkomst
- `bzgrep -H -n "zoekpatroon" pad/naar/bestand` : toon de bestandsnaam en het regelnummer voor elke overeenkomst
- `bzgrep -o "zoekpatroon" pad/naar/bestand` : zoek naar regels die overeenkomen met een patroon en geef alleen de overeenkomende tekst weer
- `cat pad/naar/bz_gecomprimeerd_bestand | bzgrep -v "zoekpatroon"` : zoek in `stdin` naar regels die niet overeenkomen met een patroon
- `bzgrep -E -i "zoekpatroon" pad/naar/bestand` : gebruik uitgebreide `regex` (ondersteunt `?`, `+`, `{}`, `()` en `|`), in hoofdletterongevoelige modus

## Links
- More information: https://manned.org/bzgrep
