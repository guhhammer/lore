# ar

Maken, aanpassen en uitpakken van Unix archieven. Typisch gebruikt voor statische bibliotheken (`.a`) en Debian pakketten (`.deb`).

## Usage
- `ar x pad/naar/bestand.a` : extraheer alles uit een archief
- `ar t pad/naar/bestand.ar` : toon inhoud van een specifiek archief
- `ar r pad/naar/bestand.deb pad/naar/debian-binary pad/naar/control.tar.gz pad/naar/data.tar.xz ...` : vervang of voeg specifieke bestanden toe aan een archief
- `ar s pad/naar/bestand.a` : voeg (s) een objectbestandsindex toe (equivalent aan het gebruik van `ranlib`)
- `ar rs pad/naar/bestand.a pad/naar/bestand1.o pad/naar/bestand2.o ...` : maak een archief met specifieke bestanden en een begeleidende objectbestandsindex

## Links
- More information: https://manned.org/ar
