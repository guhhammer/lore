# cmp

Vergelijk twee bestanden byte voor byte.

## Usage
- `cmp pad/naar/bestand1 pad/naar/bestand2` : toon karakter en regelnummer van het eerste verschil tussen twee bestanden
- `cmp -b pad/naar/bestand1 pad/naar/bestand2` : toon info van het eerste verschil: karakter, regelnummer, bytes en waardes
- `cmp -l pad/naar/bestand1 pad/naar/bestand2` : toon de bytenummers en waardes van ieder verschil
- `cmp -s pad/naar/bestand1 pad/naar/bestand2` : vergelijk bestanden, maar toon niets, pak alleen de exit status

## Links
- More information: https://www.gnu.org/software/diffutils/manual/diffutils.html#Invoking-cmp
