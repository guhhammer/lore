# indent

Wijzig het uiterlijk van een C/C++-programma door spaties in te voegen of te verwijderen.

## Usage
- `indent -linux pad/naar/bron1.c pad/naar/bron2.c ...` : formatteer C/C++-broncode volgens de Linux style guide, maak automatisch een back-up van de originele bestanden en vervang deze door de ingesprongen versies
- `indent -gnu pad/naar/bron.c -o pad/naar/ingesprongen_bron.c` : formatteer C/C++-broncode volgens de GNU-stijl en sla de ingesprongen versie op in een ander bestand
- `indent -kr -il3 -nut -l120 pad/naar/bron.c -o pad/naar/ingesprongen_bron.c` : formatteer C/C++-broncode volgens de stijl van Kernighan & Ritchie (K&R), geen tabs, 3 spaties per inspringing en breek regels af op 120 tekens

## Links
- More information: https://www.gnu.org/software/indent/manual/indent/Option-Summary.html
