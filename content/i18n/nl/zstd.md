# zstd

Bestanden comprimeren of decomprimeren met Zstandard compressie.

## Usage
- `zstd pad/naar/bestand` : comprimeer een bestand naar een nieuw bestand met de `.zst` extensie
- `zstd -d pad/naar/bestand.zst` : decomprimeer een bestand
- `zstd -d -c pad/naar/bestand.zst` : decomprimeer naar `stdout`
- `zstd -1..19 pad/naar/bestand` : comprimeer een bestand met een specifiek compressieniveau, waar 1=snelste, 19=langzaamste en 3=standaard
- `zstd --fast=1..131072 pad/naar/bestand` : comprimeer een bestand met ultrasnel compressieniveau, waarbij 1=standaard
- `zstd --ultra -1..22 pad/naar/bestand` : ontgrendel hogere compressieniveaus (tot en met 22) door gebruik te maken van meer geheugen (voor compressie en decompressie)
- `zstd -T 0` : stel het aantal werkthreads in op het aantal fysieke CPU-kernen

## Links
- More information: https://manned.org/zstd
