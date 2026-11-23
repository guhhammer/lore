# grep

Zoek patronen in bestanden met behulp van `regex`en.

## Usage
- `grep "zoekpatroon" pad/naar/bestand1 pad/naar/bestand2 ...` : zoek naar een patroon in bestanden
- `grep -F "exacte_string" pad/naar/bestand` : zoek naar een exacte string (schakelt `regex` uit)
- `grep -rI "zoekpatroon" pad/naar/map` : zoek naar een patroon in alle bestanden in een map recursief, waarbij binaire bestanden genegeerd worden
- `grep --context|--before-context|--after-context 3 "zoekpatroon" pad/naar/bestand` : toon 3 regels met Context rond, voor (B) of nA elke overeenkomst
- `grep -Hn --color=always "zoekpatroon" pad/naar/bestand` : print bestandsnaam en regelnummers voor elke overeenkomst met kleuruitvoer
- `grep -o "zoekpatroon" pad/naar/bestand` : toon alleen de overeenkomende tekst
- `cat pad/naar/bestand | grep -v "zoekpatroon"` : lees data vanuit `stdin` en toon niet de regels die overeenkomen met een patroon
- `grep -Ei "zoekpatroon" pad/naar/bestand` : gebruik uitgebreide `regex`en (ondersteunt `?`, `+`, `{}`, `()` en `|`), in hoofdletterongevoelige modus

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html
