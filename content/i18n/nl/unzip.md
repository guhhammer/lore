# unzip

Extraheer bestanden/mappen van Zip-archieven.

## Usage
- `unzip pad/naar/archief.zip` : extraheer alle bestanden/mappen van een specifiek archief naar de huidige map
- `unzip pad/naar/archief.zip -d pad/naar/uitvoer` : extraheer bestanden/mappen van een archief naar een specifieke map
- `unzip -c pad/naar/archief.zip` : extraheer bestanden/mappen van een archief naar `stdout` naast de geëxtraheerde bestandsnamen
- `unzip -O gbk pad/naar/archief.zip` : extraheer een archief dat gemaakt is op Windows, dat bestanden bevat met niet-ASCII (bijv. Chinese of Japanse tekens) bestandsnamen
- `unzip -l pad/naar/archief.zip` : toon de inhoud van een specifiek archief zonder te extraheren
- `unzip -j pad/naar/archief.zip pad/naar/bestand1_in_archief pad/naar/bestand2_in_archief ...` : extraheer specifieke bestanden uit een archief naar de huidige map, waarbij mappaden worden genegeerd

## Links
- More information: https://manned.org/unzip
