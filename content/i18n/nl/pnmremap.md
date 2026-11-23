# pnmremap

Vervang de kleuren in een PNM-afbeelding.

## Usage
- `pnmremap -ma pad/naar/kleurenpalet_bestand.ppm pad/naar/invoer.pnm >pad/naar/uitvoer.pnm` : vervang de kleuren in een afbeelding met die in het gespecificeerde kleurenpalet
- `pnmremap -ma pad/naar/kleurenpalet_bestand.ppm -fs pad/naar/invoer.pnm >pad/naar/uitvoer.pnm` : gebruik Floyd-Steinberg dithering voor het representeren van missende kleuren in het kleurenpalet
- `pnmremap -ma pad/naar/kleurenpalet_bestand.ppm -fi pad/naar/invoer.pnm >pad/naar/uitvoer.pnm` : gebruik de eerste kleur in het palet voor het representeren van missende kleuren in het kleurenpalet
- `pnmremap -ma pad/naar/kleurenpalet_bestand.ppm -m kleur pad/naar/invoer.pnm >pad/naar/uitvoer.pnm` : gebruik de gespecificeerde kleur voor het representeren van de missende kleuren in het kleurenpalet

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmremap.html
