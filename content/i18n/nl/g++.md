# g++

Compileer C++-bronbestanden. Onderdeel van GCC (GNU Compiler Collection).

## Usage
- `g++ pad/naar/bron1.cpp pad/naar/bron2.cpp ... -o pad/naar/uitvoerbaar_bestand` : compileer een bronbestand in een uitvoerbaar bestand
- `g++ pad/naar/bron.cpp -Wall -o pad/naar/uitvoerbaar_bestand` : toon alle fouten en waarschuwingen
- `g++ pad/naar/bron.cpp -Wall -g -Og -o pad/naar/uitvoerbaar_bestand` : toon veelvoorkomende waarschuwingen, debug-symbolen in de uitvoer, en optimaliseer zonder debugging te beïnvloeden
- `g++ pad/naar/bron.cpp -std=c++98|c++11|c++14|c++17 -o pad/naar/uitvoerbaar_bestand` : kies een taalstandaard om mee te compileren (C++98/C++11/C++14/C++17)
- `g++ pad/naar/bron.cpp -o pad/naar/uitvoerbaar_bestand -Ipad/naar/header -Lpad/naar/bibliotheek -lbibliotheek_naam` : voeg bibliotheken toe die zich op een ander pad bevinden dan het bronbestand
- `g++ -c pad/naar/bron1.cpp pad/naar/bron2.cpp ... && g++ -o pad/naar/uitvoerbaar_bestand pad/naar/bron1.o pad/naar/bron2.o ...` : compileer en link meerdere bronbestanden tot één uitvoerbaar bestand
- `g++ pad/naar/bron.cpp -O1|2|3|fast -o pad/naar/uitvoerbaar_bestand` : optimaliseer het gecompileerde programma voor prestaties
- `g++ --version` : toon de versie

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/C_002b_002b-Dialect-Options.html
