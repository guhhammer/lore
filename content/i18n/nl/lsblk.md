# lsblk

Toon informatie over apparaten.

## Usage
- `lsblk` : toon alle opslagapparaten in een boomstructuur
- `lsblk -a` : toon ook lege apparaten
- `lsblk -b` : toon de SIZE-kolom in bytes in plaats van in een leesbaar formaat
- `lsblk -f` : toon informatie over bestandssystemen
- `lsblk -i` : gebruik ASCII-tekens voor de boomstructuur
- `lsblk -t` : toon informatie over de topologie van blokapparaten
- `lsblk -e 1,7,...` : sluit de apparaten uit die zijn opgegeven met een door komma's gescheiden lijst met hoofdapparaatnummers
- `lsblk -o +NAME,ROTA,SERIAL,MODEL,TRAN,TYPE,SIZE,FSTYPE,MOUNTPOINT,...` : voeg extra informatie toe aan de uitvoer gebruikmakend van een door komma's gescheiden lijst met kolommen (laat het `+`-teken weg om alleen de opgegeven kolommen te tonen)

## Links
- More information: https://manned.org/lsblk
