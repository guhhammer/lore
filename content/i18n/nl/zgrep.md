# zgrep

Zoek tekstpatronen in bestanden binnen gecomprimeerde bestanden.

## Usage
- `zgrep patroon pad/naar/gecomprimeerd_bestand` : zoek een patroon in een gecomprimeerd bestand (hoofdlettergevoelig)
- `zgrep --context|--before-context|--after-context 3 patroon pad/naar/gecomprimeerd_bestand` : toon 3 regels rondom Context, voor (B) of nA elke overeenkomst
- `zgrep -i patroon pad/naar/gecomprimeerd_bestand` : zoek een patroon in een gecomprimeerd bestand (hoofdletterongevoelig)
- `zgrep -c patroon pad/naar/gecomprimeerd_bestand` : geef het aantal regels met het gevonden patroon in een gecomprimeerd bestand weer
- `zgrep -v patroon pad/naar/gecomprimeerd_bestand` : geef de regels weer waarin het patroon niet voorkomt (de zoekfunctie omkeren)
- `zgrep -e "patroon_1" -e "patroon_2" pad/naar/gecomprimeerd_bestand` : zoek in een gecomprimeerd bestand naar meerdere patronen
- `zgrep -E regex pad/naar/bestand` : gebruik uitgebreide `regex` (ondersteunt `?`, `+`, `{}`, `()` en `|`)

## Links
- More information: https://manned.org/zgrep
