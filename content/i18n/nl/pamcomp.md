# pamcomp

Leg twee PAM afbeeldingen over elkaar.

## Usage
- `pamcomp pad/naar/bovenlaag.pam pad/naar/onderlaag.pam >pad/naar/uitvoer.pam` : leg twee afbeeldingen over elkaar zodat de bovenlaag delen van de onderlaag blokkeert
- `pamcomp -ali left|center|right|beyondleft|beyondright -x x_offset pad/naar/bovenlaag.pam pad/naar/onderlaag.pam >pad/naar/uitvoer.pam` : zet de horizontale uitlijning van de bovenlaag
- `pamcomp -va top|middle|bottom|above|below -y y_offset pad/naar/bovenlaag.pam pad/naar/onderlaag.pam >pad/naar/uitvoer.pam` : zet de verticale uitlijning van de bovenlaag
- `pamcomp -o 0.7 pad/naar/bovenlaag.pam pad/naar/onderlaag.pam >pad/naar/uitvoer.pam` : zet de dekking van de bovenlaag

## Links
- More information: https://netpbm.sourceforge.net/doc/pamcomp.html
