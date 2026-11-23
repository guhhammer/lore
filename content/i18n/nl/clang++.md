# clang++

Compileert C++ bronbestanden. Onderdeel van LLVM.

## Usage
- `clang++ pad/naar/bron1.cpp pad/naar/bron2.cpp ... -o pad/naar/uitvoerbaar_bestand` : compileer broncodebestand(en) naar een uitvoerbaar binair bestand
- `clang++ pad/naar/bron.cpp -Wall -o pad/naar/uitvoerbaar_bestand` : toon (bijna) alle fouten en waarschuwingen
- `clang++ pad/naar/bron.cpp -Wall -g -Og -o pad/naar/uitvoerbaar_bestand` : toon veelvoorkomende waarschuwingen, debug-symbolen in de uitvoer, en optimaliseer zonder debugging te beïnvloeden
- `clang++ pad/naar/bron.cpp -std=c++20 -o pad/naar/uitvoerbaar_bestand` : kies een taalstandaard om mee te compileren
- `clang++ pad/naar/bron.cpp -o pad/naar/uitvoerbaar_bestand -Ipad/naar/header_pad -Lpad/naar/bibliotheek_pad -lpad/naar/bibliotheek_naam` : voeg bibliotheken toe die zich op een ander pad bevinden dan het bronbestand
- `clang++ -S -emit-llvm pad/naar/bron.cpp -o pad/naar/uitvoer.ll` : compileer broncode naar LLVM Intermediate Representation (IR)
- `clang++ pad/naar/bron.cpp -O1|2|3|fast -o pad/naar/uitvoerbaar_bestand` : optimaliseer het gecompileerde programma voor prestaties
- `clang++ --version` : toon de versie

## Links
- More information: https://clang.llvm.org/docs/UsersManual.html#command-line-options
