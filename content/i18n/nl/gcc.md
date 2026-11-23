# gcc

Preprocess en compileer C en C++ bronbestanden, monteer en koppel ze vervolgens samen. Onderdeel van GCC (GNU Compiler Collection).

## Usage
- `gcc pad/naar/bron1.c pad/naar/bron2.c ... -o pad/naar/uitvoerbaar_bestand` : compileer meerdere bronbestanden in een uitvoerbaar bestand
- `gcc pad/naar/bron.c -Wall -o pad/naar/uitvoerbaar_bestand` : toon alle fouten en waarschuwingen
- `gcc pad/naar/bron.c -Wall -g -Og -o pad/naar/uitvoerbaar_bestand` : toon veelvoorkomende waarschuwingen, debug-symbolen in de uitvoer, en optimaliseer zonder debugging te beïnvloeden
- `gcc pad/naar/bron.c -o pad/naar/uitvoerbaar_bestand -Ipad/naar/header -Lpad/naar/bibliotheek -lbibliotheek_naam` : voeg bibliotheken toe die zich op een ander pad bevinden dan het bronbestand
- `gcc -S pad/naar/bron.c` : compileer broncode naar Assembler-instructies
- `gcc -c pad/naar/bron.c` : compileer broncode naar een objectbestand zonder deze te linken
- `gcc pad/naar/bron.c -O1|2|3|fast -o pad/naar/uitvoerbaar_bestand` : optimaliseer het gecompileerde programma voor prestaties
- `gcc --version` : toon de versie

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/
