# pamditherbw

Pas dithering toe op een grijsschaalafbeelding, d.w.z. zet deze om in een patroon van zwarte en witte pixels die eruitzien als de originele grijsschaal.

## Usage
- `pamditherbw pad/naar/afbeelding.pgm >pad/naar/bestand.pgm` : lees een PGM afbeelding, pas dithering toe en sla deze op in een bestand
- `pamditherbw -floyd|fs|atkinson|threshold|hilbert|... pad/naar/afbeelding.pgm >pad/naar/bestand.pgm` : gebruik de gespecificeerde kwantisatiemethode
- `pamditherbw -a -r 1337 pad/naar/afbeelding.pgm >pad/naar/bestand.pgm` : gebruik de atkinson kwantisatiemethode en de gespecificeerde seed voor een pseudo-random getallengenerator
- `pamditherbw -fs|atkinson|thresholding -va 0.3 pad/naar/afbeelding.pgm >pad/naar/bestand.pgm` : specificeer de drempelwaarde van de kwantisatiemethodes die een vorm van drempels uitvoeren

## Links
- More information: https://netpbm.sourceforge.net/doc/pamditherbw.html
