# look

Toon regels die beginnen met een prefix in een gesorteerd bestand. Opmerking: de regels in het bestand moeten gesorteerd zijn.

## Usage
- `look prefix pad/naar/bestand` : zoek naar regels die beginnen met een specifieke prefix in een specifiek bestand
- `look -f -d prefix pad/naar/bestand` : zoek hoofdletterongevoelig alleen op alfanumerieke tekens
- `look -t ,` : specificeer een karakter voor het beëindigen van een string (standaard een spatie)
- `look prefix` : zoek in `/usr/share/dict/words` (`--alphanum` en `--ignore-case` worden verondersteld)

## Links
- More information: https://man.openbsd.org/look
