# readarray

Lees regels van `stdin` in een array.

## Usage
- `readarray array_naam` : voer interactief regels in een array in
- `readarray <pad/naar/bestand.txt array_naam` : lees regels uit een bestand en plaats ze in een array
- `readarray <pad/naar/bestand.txt -t array_naam` : verwijder scheidingstekens aan het einde (t) (standaard newline)
- `readarray <pad/naar/bestand.txt -n n array_naam` : kopieer maximaal `n` regels
- `readarray <pad/naar/bestand.txt -s n array_naam` : sla de eerste `n` regels over
- `readarray <pad/naar/bestand.txt -d scheidingsteken array_naam` : zet een aangepast scheidingsteken
- `help mapfile` : toon de help

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-readarray
