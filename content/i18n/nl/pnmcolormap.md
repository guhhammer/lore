# pnmcolormap

Maak een kwantisatiekleurkaart voor een PNM-afbeelding.

## Usage
- `pnmcolormap n_kleuren pad/naar/invoer.pnm >pad/naar/uitvoer.ppm` : genereer een afbeelding met alleen `n_kleuren` of minder kleuren, zo dicht als mogelijk bij de invoerafbeelding
- `pnmcolormap -splits n_kleuren pad/naar/invoer.pnm >pad/naar/uitvoer.ppm` : gebruik de splitspread strategie voor het bepalen van de uitvoer-kleuren, welke mogelijk een beter resultaat oplevert met afbeeldingen met kleine details
- `pnmcolormap -so pad/naar/invoer.pnm >pad/naar/uitvoer.ppm` : sorteer de resulterende kleurkaart, welke nuttig is voor het vergelijken van kleurkaarten

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmcolormap.html
