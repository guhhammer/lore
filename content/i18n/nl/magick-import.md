# magick import

Leg een deel of het geheel van een X-serverscherm vast en sla de afbeelding op in een bestand.

## Usage
- `magick import -window root pad/naar/uitvoer.ps` : leg het hele X-serverscherm vast in een PostScript-bestand
- `magick import -window root -display externe_host:scherm.display pad/naar/uitvoer.png` : leg de inhoud van een extern X-serverscherm vast in een PNG-afbeelding
- `magick import -window window_id pad/naar/uitvoer.jpg` : leg een specifiek venster vast op basis van zijn ID zoals weergegeven door `xwininfo` in een JPEG-afbeelding

## Links
- More information: https://imagemagick.org/script/import.php
