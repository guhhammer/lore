# magick convert

Converteer tussen afbeeldingsformaten, schaal, voeg samen, maak afbeeldingen en nog veel meer. Opmerking: deze tool (voorheen `convert`) is vervangen door `magick` in ImageMagick 7+.

## Usage
- `magick convert pad/naar/invoer_afbeelding.jpg pad/naar/uitvoer_afbeelding.png` : converteer een afbeelding van JPEG naar PNG
- `magick convert pad/naar/invoer_afbeelding.png -resize 50% pad/naar/uitvoer_afbeelding.png` : schaal een afbeelding naar 50% van zijn originele grootte
- `magick convert pad/naar/invoer_afbeelding.png -resize 640x480 pad/naar/uitvoer_afbeelding.png` : schaal een afbeelding en behoud de originele aspect ratio tot een maximale dimensie van 640x480
- `magick convert pad/naar/invoer_afbeelding.png -define jpeg:extent=512kb pad/naar/uitvoer_afbeelding.jpg` : schaal een afbeelding zodat deze een gespecificeerde bestandsgrootte heeft
- `magick convert -background none pad/naar/afbeelding1.png pad/naar/afbeelding2.png ... -append|+append pad/naar/uitvoer_afbeelding.png` : voeg afbeeldingen verticaal/horizontaal samen en maak de lege ruimte transparant
- `magick convert pad/naar/afbeelding1.png pad/naar/afbeelding2.png ... -delay 10 pad/naar/animatie.gif` : maak een GIF van een reeks van afbeeldingen met 100ms pauze ertussen
- `magick convert -size 800x600 "xc:#ff0000" pad/naar/afbeelding.png` : maak een afbeelding met niets anders dan een volledig rode achtergrond
- `magick convert pad/naar/afbeelding1.png pad/naar/afbeelding2.png ... pad/naar/favicon.ico` : maak een favicon van verschillende afbeeldingen met verschillende groottes

## Links
- More information: https://imagemagick.org/script/convert.php
