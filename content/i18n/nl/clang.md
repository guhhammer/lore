# clang

Compileer C, C++, en Objective-C bronbestanden. Kan gebruikt worden als een vervanger van GCC. Onderdeel van LLVM.

## Usage
- `clang pad/naar/bron1.c pad/naar/bron2.c ... -o pad/naar/uitvoerbaar_bestand` : compileer broncodebestand(en) naar een uitvoerbaar binair bestand
- `clang pad/naar/bron.c -Wall -o pad/naar/uitvoerbaar_bestand` : toon (bijna) alle fouten en waarschuwingen
- `clang pad/naar/bron.c -Wall -g -Og -o pad/naar/uitvoerbaar_bestand` : toon veelvoorkomende waarschuwingen, debug-symbolen in de uitvoer, en optimaliseer zonder debugging te beïnvloeden
- `clang pad/naar/bron.c -o pad/naar/uitvoerbaar_bestand -Ipad/naar/header -Lpad/naar/bibliotheek -lbibliotheek_naam` : voeg bibliotheken toe die zich op een ander pad bevinden dan het bronbestand
- `clang -S -emit-llvm pad/naar/bron.c -o pad/naar/uitvoer.ll` : compileer broncode naar LLVM Intermediate Representation (IR)
- `clang -c pad/naar/bron.c` : compileer broncode zonder deze te linken
- `clang pad/naar/bron.c -O1|2|3|fast -o pad/naar/uitvoerbaar_bestand` : optimaliseer het gecompileerde programma voor prestaties
- `clang --version` : toon de versie

## Links
- More information: https://clang.llvm.org/docs/ClangCommandLineReference.html
