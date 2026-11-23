# xz

Comprimeren of decomprimeren van XZ en LZMA bestanden.

## Usage
- `xz pad/naar/bestand` : comprimeer een bestand naar het XZ-formaat
- `xz -d pad/naar/bestand.xz` : decomprimeer een XZ-bestand
- `xz -F lzma pad/naar/bestand` : comprimeer een bestand gebruikmakend van LZMA
- `xz -d -F lzma pad/naar/bestand.lzma` : decomprimeer een LZMA-bestand
- `xz -d -c pad/naar/bestand.xz` : decomprimeer een bestand en schrijf het naar `stdout` (impliceert `--keep`)
- `xz -k pad/naar/bestand` : comprimeer een bestand, maar verwijder het origineel niet
- `xz -0 pad/naar/bestand` : comprimeer een bestand, gebruikmakend van de snelste compressie
- `xz -9 pad/naar/bestand` : comprimeer een bestand, gebruikmakend van de beste compressie

## Links
- More information: https://manned.org/xz
