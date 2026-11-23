# pamundice

Combineer een raster van Netpbm-afbeeldingen tot één afbeelding.

## Usage
- `pamundice bestandsnaam_%1d_%1a.ppm -a raster_breedte -d raster_hoogte >pad/naar/uitvoer.ppm` : combineer de afbeeldingen waarvan de namen overeenkomen met de `printf`-stijl bestandsnaam-expressie. Ga uit van een raster met een specifieke afmeting
- `pamundice bestandsnaam_%1d_%1a.ppm -a x_waarde -d y_waarde -ho waarde -vo waarde >pad/naar/uitvoer.ppm` : ga ervan uit dat de tegels horizontaal en verticaal overlappen met het gespecificeerde bedrag
- `pamundice -l pad/naar/bestand.txt -a x_waarde -d y_waarde >pad/naar/uitvoer.ppm` : specificeer de te combineren afbeeldingen via een tekstbestand met één bestandsnaam per regel

## Links
- More information: https://netpbm.sourceforge.net/doc/pamundice.html
