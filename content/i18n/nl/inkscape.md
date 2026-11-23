# inkscape

Een SVG (Scalable Vector Graphics) bewerkingsprogramma. Voor Inkscape versies tot en met 0.92.x, gebruik -e in plaats van -o.

## Usage
- `inkscape pad/naar/bestand.svg` : open een SVG-bestand in de Inkscape GUI
- `inkscape pad/naar/bestand.svg -o pad/naar/bestand.png` : exporteer een SVG-bestand in een bitmap met het standaardformaat (PNG) en de standaardresolutie (96 DPI)
- `inkscape pad/naar/bestand.svg -o pad/naar/bestand.png -w 600 -h 400` : exporteer een SVG-bestand in een bitmap van 600x400 pixels (vervorming van de aspectverhouding mogelijk)
- `inkscape pad/naar/bestand.svg -o pad/naar/bestand.png -D` : exporteer de tekening (selectiekader van alle objecten) van een SVG-bestand in een bitmap
- `inkscape pad/naar/bestand.svg -i object_id -o object.png` : exporteer een enkel object, gezien zijn ID, in een bitmap
- `inkscape pad/naar/bestand.svg -o pad/naar/bestand.pdf -T` : exporteer een SVG-document naar PDF, converteer alle teksten naar paden
- `inkscape pad/naar/bestand.svg --select=path123 --verb="EditDuplicate;ObjectRotate90;FileSave;FileQuit"` : dupliceer het object met id="path123", roteer het duplicaat 90 graden, sla het bestand op, en sluit Inkscape af

## Links
- More information: https://inkscape.org/doc/inkscape-man.html
