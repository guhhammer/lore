# lrzip

Een programma voor het comprimeren van grote bestanden.

## Usage
- `lrzip pad/naar/bestand` : comprimeer een bestand met LZMA - langzame compressie, snelle decompressie
- `lrzip -b pad/naar/bestand` : comprimeer een bestand met BZIP2 - goede middenweg voor compressie/snelheid
- `lrzip -z pad/naar/bestand` : comprimeer met ZPAQ - extreme compressie, maar erg langzaam
- `lrzip -l pad/naar/bestand` : comprimeer met LZO - lichte compressie, extreem snelle decompressie
- `lrzip -e pad/naar/bestand` : comprimeer een bestand en beveilig/versleutel het met een wachtwoord
- `lrzip -p 8 pad/naar/bestand` : overschrijf het aantal processorthreads om te gebruiken

## Links
- More information: https://manned.org/lrzip
