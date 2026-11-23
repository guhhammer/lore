# restorecon

Herstel de SELinux-beveiligingscontext op bestanden/mappen volgens persistente regels.

## Usage
- `ls -dlZ pad/naar/bestand_of_map` : toon de huidige beveiligingscontext van een bestand of map
- `restorecon pad/naar/bestand_of_map` : herstel de beveiligingscontext van een bestand of map
- `restorecon -R -v pad/naar/map` : herstel de beveiligingscontext van een map recursief en toon alle gewijzigde labels
- `restorecon -R -T 0 -p pad/naar/map` : herstel de beveiligingscontext van een map recursief, met gebruik van alle beschikbare threads, en toon voortgang
- `restorecon -R -n -v pad/naar/map` : bekijk vooraf welke labelwijzigingen zouden plaatsvinden zonder ze toe te passen

## Links
- More information: https://manned.org/restorecon
