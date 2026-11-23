# lrztar

Een wrapper voor `lrzip` om het comprimeren van mappen te vereenvoudigen.

## Usage
- `lrztar pad/naar/map` : archiveer een map met `tar` en comprimeer dan
- `lrztar -z pad/naar/map` : archiveer een map met `tar` en comprimeer dan met ZPAQ (extreme compressie, maar erg langzaam)
- `lrztar -o pad/naar/bestand pad/naar/map` : geef het uitvoerbestand op
- `lrztar -p 8 pad/naar/map` : overschrijf het aantal processorthreads dat gebruikt moet worden
- `lrztar -f pad/naar/map` : forceer het overschrijven van bestaande bestanden

## Links
- More information: https://manned.org/lrztar
