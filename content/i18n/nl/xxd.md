# xxd

Maak een hexadecimale representatie (hexdump) van een binair bestand, of andersom.

## Usage
- `xxd invoerbestand` : genereer een hexdump van een binair bestand en toon de uitvoer
- `xxd invoerbestand uitvoerbestand` : genereer een hexdump van een binair bestand en sla het op als een tekstbestand
- `xxd -a invoerbestand` : toon een compactere uitvoer, waarbij opeenvolgende nullen (indien aanwezig) vervangen worden door een sterretje
- `xxd -c 10 invoerbestand` : toon de uitvoer met 10 kolommen van elk één octet (byte)
- `xxd -l 32 invoerbestand` : toon alleen uitvoer tot een lengte van 32 bytes
- `xxd -p invoerbestand` : toon de uitvoer in platte modus, zonder ruimtes tussen de kolommen
- `xxd -r -p invoerbestand uitvoerbestand` : zet een platte-tekst hexdump terug om naar binair en sla het op als een binair bestand

## Links
- More information: https://manned.org/xxd
