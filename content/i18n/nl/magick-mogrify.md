# magick mogrify

Voer bewerkingen uit op meerdere afbeeldingen, zoals het wijzigen van de grootte, bijsnijden, omkeren en effecten toevoegen. Wijzigingen worden direct toegepast op het originele bestand.

## Usage
- `magick mogrify -resize 50% *.jpg` : wijzig de grootte van alle JPEG-afbeeldingen in de map naar 50% van hun oorspronkelijke grootte
- `magick mogrify -resize 800x600 DSC*` : wijzig de grootte van alle afbeeldingen die beginnen met `DSC` naar 800x600
- `magick mogrify -format jpg *.png` : converteer alle PNG's in de map naar JPEG
- `magick mogrify -modulate 100,50 *` : halveer de verzadiging van alle afbeeldingsbestanden in de huidige map
- `magick mogrify -modulate 200 *` : verdubbel de helderheid van alle afbeeldingsbestanden in de huidige map
- `magick mogrify -layers 'optimize' -fuzz 7% *.gif` : verklein de bestandsgrootte van alle GIF-afbeeldingen in de huidige map door de kwaliteit te verlagen
- `magick mogrify -help` : toon de help

## Links
- More information: https://imagemagick.org/script/mogrify.php
