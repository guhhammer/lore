# magick

Creëer, bewerk, stel samen, of converteer tussen afbeeldingsformaten. Deze tool vervangt `convert` in ImageMagick 7+. Bekijk `magick convert` om de oude tool te gebruiken in versies 7+. Sommige subcommando's zoals `mogrify` hebben hun eigen documentatie.

## Usage
- `magick pad/naar/invoer_afbeelding.png pad/naar/uitvoer_afbeelding.jpg` : converteer tussen afbeeldingsformaten
- `magick pad/naar/invoer_afbeelding.jpg -resize 100x100 pad/naar/uitvoer_afbeelding.jpg` : wijzig de grootte van een afbeelding en maak een nieuwe kopie
- `magick pad/naar/invoer_afbeelding.png -resize 50% pad/naar/uitvoer_afbeelding.png` : wijzig de grootte van een afbeelding met een percentage
- `magick pad/naar/invoer_afbeelding.png -define jpeg:extent=512kb pad/naar/uitvoer_afbeelding.jpg` : schaal een afbeelding naar een bepaalde bestandsgrootte
- `magick convert -background none pad/naar/afbeelding1.png pad/naar/afbeelding2.png ... -append|+append pad/naar/uitvoer_afbeelding.png` : voeg afbeeldingen verticaal/horizontaal samen en maak de lege ruimte transparant
- `magick *.jpg pad/naar/afbeeldingen.gif` : maak een GIF van alle JPEG-afbeeldingen uit de huidige map
- `magick -size 640x480 pattern:checkerboard pad/naar/dambordpatroon.png` : creëer een dambordpatroon
- `magick *.jpg -adjoin pad/naar/bestand.pdf` : maak een PDF van alle JPEG-afbeeldingen uit de huidige map

## Links
- More information: https://imagemagick.org/script/magick.php
