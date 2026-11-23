# pamtotiff

Converteer een PAM afbeelding naar een TIFF-bestand.

## Usage
- `pamtotiff pad/naar/invoer_bestand.pam >pad/naar/uitvoer_bestand.tiff` : converteer een PAM afbeelding naar een TIFF afbeelding
- `pamtotiff -none|packbits|lzw|g3|g4|flate|adobeflate pad/naar/invoer_bestand.pam >pad/naar/uitvoer_bestand.tiff` : specificeer expliciet de compressiemethode voor een uitvoerbestand
- `pamtotiff -c pad/naar/invoer_bestand.pam >pad/naar/uitvoer_bestand.tiff` : produceer altijd een gekleurde TIFF afbeelding, ook als de invoerafbeelding een grijsschaal is

## Links
- More information: https://netpbm.sourceforge.net/doc/pamtotiff.html
