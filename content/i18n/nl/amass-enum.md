# amass enum

Vind subdomeinen van een domein.

## Usage
- `amass enum -d domeinnaam` : vind, passief, subdomeinen van een domein
- `amass enum -active -d domeinnaam -p 80,443,8080` : zoek subdomeinen van een domein en verifieer ze actief in een poging de gevonden subdomeinen op te lossen
- `amass enum -brute -d domeinnaam` : doe een brute force zoekopdracht naar subdomeinen
- `amass enum -o uitvoer_bestand -d domeinnaam` : sla de resultaten op in een tekstbestand
- `amass enum -o uitvoer_bestand -dir pad/naar/map -d domeinnaam` : sla de terminaluitvoer op in een bestand en andere gedetailleerde uitvoer in een map
- `amass enum -list` : toon alle beschikbare databronnen
