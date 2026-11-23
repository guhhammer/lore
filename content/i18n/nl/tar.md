# tar

Archiveringsprogramma. Vaak gecombineerd met een compressiemethode, zoals `gzip` of `bzip2`.

## Usage
- `tar cf pad/naar/doel.tar pad/naar/bestand1 pad/naar/bestand2 ...` : creëer een archief en schrijf het naar een bestand (f)
- `tar czf pad/naar/doel.tar.gz pad/naar/bestand1 pad/naar/bestand2 ...` : creëer een gzipt archief en schrijf het naar een bestand (f)
- `tar czf pad/naar/doel.tar.gz -C pad/naar/map .` : creëer een gzipt (gecomprimeerd) archief van een map met relatieve paden
- `tar xvf pad/naar/bron.tar.ext` : extraheer een (gecomprimeerd) archiefbestand (f) naar de huidige map verbose
- `tar xf pad/naar/bron.tar.ext -C pad/naar/map` : extraheer een (gecomprimeerd) archiefbestand (f) naar de doelmap
- `tar caf pad/naar/doel.tar.xz pad/naar/bestand1 pad/naar/bestand2 ...` : creëer een gecomprimeerd archief en schrijf het naar een bestand (f), gebruikmakend van de bestandsextensie om automatisch het compressieprogramma te bepalen
- `tar tvf pad/naar/bron.tar` : toon de inhoud van een tarbestand (f) verbose
- `tar xf pad/naar/bron.tar --wildcards "*.html"` : extraheer bestanden die overeenkomen met een patroon uit een archiefbestand (f)

## Links
- More information: https://www.gnu.org/software/tar/manual/tar.html
