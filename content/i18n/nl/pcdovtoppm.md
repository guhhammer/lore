# pcdovtoppm

Maak een indexafbeelding voor een fotocd op basis van het overzichtsbestand.

## Usage
- `pcdovtoppm pad/naar/bestand.pcd >pad/naar/uitvoer.ppm` : maak een PPM-indexafbeelding van een PCD-overzichtsbestand
- `pcdovtoppm -m breedte -s grootte pad/naar/bestand.pcd >pad/naar/uitvoer.ppm` : specificeer de maximale breedte van de uitvoerafbeelding en de maximale grootte van elke afbeelding die in de uitvoer wordt opgenomen
- `pcdovtoppm -a n_afbeeldingen -c n_kleuren pad/naar/bestand.pcd >pad/naar/uitvoer.ppm` : specificeer het maximale aantal afbeeldingen naast elkaar en het maximale aantal kleuren
- `pcdovtoppm -f lettertype -w pad/naar/bestand.pcd >pad/naar/uitvoer.ppm` : gebruik het gespecificeerde lettertype voor annotaties en kleur de achtergrond wit

## Links
- More information: https://netpbm.sourceforge.net/doc/pcdovtoppm.html
