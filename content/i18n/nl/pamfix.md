# pamfix

Repareer fouten in PAM, PBM, PGM en PPM bestanden.

## Usage
- `pamfix -t pad/naar/corrupt.ext >pad/naar/uitvoer.ext` : repareer een Netpbm bestand dat zijn laatste deel mist
- `pamfix -cl pad/naar/corrupt.ext >pad/naar/uitvoer.ext` : repareer een Netpbm bestand waar de pixelwaardes de `maxval` van de afbeelding overschrijden door de overtredende pixels te verlagen in waarde
- `pamfix -ch pad/naar/corrupt.pam|pbm|pgm|ppm >pad/naar/uitvoer.pam|pbm|pgm|ppm` : repareer een Netpbm bestand waar de pixelwaardes de `maxval` van de afbeelding overschrijden door deze te verhogen

## Links
- More information: https://netpbm.sourceforge.net/doc/pamfix.html
