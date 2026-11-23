# pngtopam

Converteer een PNG-afbeelding naar een Netpbm-afbeelding.

## Usage
- `pngtopam pad/naar/afbeelding.png >pad/naar/uitvoer.pam` : converteer de gespecificeerde PNG-afbeelding naar een Netpbm-afbeelding
- `pngtopam -alphapam pad/naar/afbeelding.png >pad/naar/uitvoer.pam` : maak een uitvoerafbeelding die zowel de hoofdafbeelding als het transparantiemasker van de invoerafbeelding bevat
- `pngtopam -m -ba kleur pad/naar/afbeelding.png >pad/naar/uitvoer.pam` : vervang transparante pixels door de gespecificeerde kleur
- `pngtopam -te pad/naar/bestand.txt pad/naar/afbeelding.png >pad/naar/uitvoer.pam` : schrijf tEXt chunks gevonden in de invoerafbeelding naar het gespecificeerde tekstbestand

## Links
- More information: https://netpbm.sourceforge.net/doc/pngtopam.html
