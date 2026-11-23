# pamtowinicon

Converteer een PAM afbeelding naar een Windows ICO-bestand.

## Usage
- `pamtowinicon pad/naar/invoer_bestand.pam >pad/naar/uitvoer.ico` : converteer een PAM afbeelding naar een ICO-bestand
- `pamtowinicon -pn t pad/naar/invoer_bestand.pam >pad/naar/uitvoer.ico` : encodeer afbeeldingen met resoluties kleiner dan `t` in het BMP-formaat en alle andere afbeeldingen in het PNG-formaat
- `pamtowinicon -t pad/naar/invoer_bestand.pam >pad/naar/uitvoer.ico` : maak alle pixels buiten het doorzichtige vlak zwart

## Links
- More information: https://netpbm.sourceforge.net/doc/pamtowinicon.html
