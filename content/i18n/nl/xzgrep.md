# xzgrep

Zoek bestanden die mogelijk gecomprimeerd zijn met `xz`, `lzma`, `gzip`, `bzip2`, `lzop`, of `zstd` met behulp van `regex`.

## Usage
- `xzgrep "zoekpatroon" pad/naar/bestand` : zoek naar een patroon in een bestand
- `xzgrep -F "exacte_string" pad/naar/bestand` : zoek naar een exacte tekenreeks (schakelt `regex` uit)
- `xzgrep -n "zoekpatroon" pad/naar/bestand` : zoek naar een patroon in alle bestanden en geef de regelnummers weer van de overeenkomsten
- `xzgrep --context|--before-context|--after-context 3 "zoekpatroon" pad/naar/bestand` : toon 3 regels rondom Context, voor (B) of nA elke overeenkomst
- `xzgrep -H -n --color=always "zoekpatroon" pad/naar/bestand` : toon bestandsnaam en regelnummer voor elke overeenkomst met kleuren
- `xzgrep -o "zoekpatroon" pad/naar/bestand` : zoek naar regels die overeenkomen met een patroon en toon alleen de gematchte tekst
- `xzgrep -E -i "zoekpatroon" pad/naar/bestand` : gebruik uitgebreide `regex` (ondersteunt `?`, `+`, `{}`, `()` en `|`), in hoofdletterongevoelige modus

## Links
- More information: https://manned.org/xzgrep
