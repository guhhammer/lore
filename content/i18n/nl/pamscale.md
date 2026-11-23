# pamscale

Schaal een Netpbm-afbeelding.

## Usage
- `pamscale -wid breedte -h hoogte pad/naar/invoer.pam >pad/naar/uitvoer.pam` : schaal een afbeelding zodat het resultaat de gespecificeerde afmetingen heeft
- `pamscale -wid breedte pad/naar/invoer.pam >pad/naar/uitvoer.pam` : schaal een afbeelding zodat het resultaat de gespecificeerde breedte heeft met behoud van de beeldverhouding
- `pamscale -xsc x_factor -ysc y_factor pad/naar/invoer.pam >pad/naar/uitvoer.pam` : schaal een afbeelding zodat de breedte en de hoogte aangepast worden volgens de gespecificeerde factoren
- `pamscale -xyfit kader_breedte kader_hoogte pad/naar/invoer.pam >pad/naar/uitvoer.pam` : schaal een afbeelding zodat het past binnen het gespecificeerde kader met behoud van de beeldverhouding
- `pamscale -xyfill rechthoek_breedte rechthoek_hoogte pad/naar/invoer.pam >pad/naar/uitvoer.pam` : schaal een afbeelding zodat het de gespecificeerde rechthoek volledig vult met behoud van de beeldverhouding

## Links
- More information: https://netpbm.sourceforge.net/doc/pamscale.html
