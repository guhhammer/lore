# pnmnorm

Normaliseer het contrast in een PNM-afbeelding.

## Usage
- `pnmnorm pad/naar/afbeelding.pnm >pad/naar/uitvoer.pnm` : forceer de helderste pixels om wit te zijn, de donkerste pixels om zwart te zijn en verspreid de tussenliggende pixels lineair
- `pnmnorm -midv n pad/naar/afbeelding.pnm >pad/naar/uitvoer.pnm` : forceer de helderste pixels om wit te zijn, de donkerste pixels om zwart te zijn en verspreid de tussenliggende pixels kwadratisch zodat pixels met een helderheid van `n` 50% helder worden
- `pnmnorm -k pad/naar/afbeelding.pnm >pad/naar/uitvoer.pnm` : behoud de tint van de pixels, pas alleen de helderheid aan
- `pnmnorm -luminosity|colorvalue|saturation pad/naar/afbeelding.pnm >pad/naar/uitvoer.pnm` : specificeer een methode om de helderheid van een pixel te berekenen

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmnorm.html
