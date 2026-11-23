# cat

Toon en voeg bestanden samen.

## Usage
- `cat pad/naar/bestand` : toon de inhoud van een bestand in `stdout`
- `cat pad/naar/bestand1 pad/naar/bestand2 ... >pad/naar/uitvoerbestand` : voeg verschillende bestanden samen in een uitvoerbestand
- `cat pad/naar/bestand1 pad/naar/bestand2 ... >>pad/naar/uitvoerbestand` : voeg verschillende bestanden toe aan een uitvoerbestand
- `cat -u /dev/tty12 >/dev/tty13` : kopieer de inhoud van een bestand in een uitvoerbestand zonder te bufferen
- `cat - >pad/naar/bestand` : schrijf `stdin` naar een bestand

## Links
- More information: https://manned.org/cat.1posix
