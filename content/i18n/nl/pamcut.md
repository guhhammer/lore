# pamcut

Snij een rechthoekig gebied uit van een Netpbm-afbeelding.

## Usage
- `pamcut -cropl waarde -cropr waarde -cropt waarde -cropb waarde pad/naar/afbeelding.ppm >pad/naar/uitvoer.ppm` : verwijder het gespecificeerde aantal kolommen/rijen van iedere zijde van de afbeelding
- `pamcut -l waarde -ri waarde pad/naar/afbeelding.ppm >pad/naar/uitvoer.ppm` : behoud alleen de kolommen tussen de gespecificeerde kolommen (inclusief de gespecificeerde)
- `pamcut -t waarde -b waarde -pad pad/naar/afbeelding.ppm >pad/naar/uitvoer.ppm` : vul missende gebieden met zwarte pixels als de gespecificeerde rechthoek niet volledig ligt in de invoerafbeelding

## Links
- More information: https://netpbm.sourceforge.net/doc/pamcut.html
