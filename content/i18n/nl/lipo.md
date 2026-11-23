# lipo

Tool voor het verwerken van Mach-O Universal Binaries.

## Usage
- `lipo pad/naar/binary_bestand.x86_64 pad/naar/binary_bestand.arm64e -create -output pad/naar/binary_bestand` : maak een universeel bestand uit twee bestanden met één architectuur
- `lipo pad/naar/binary_bestand -archs` : toon alle architecturen die een universeel bestand bevat
- `lipo pad/naar/binary_bestand -detailed_info` : toon gedetailleerde informatie over een universeel bestand
- `lipo pad/naar/binary_bestand -thin arm64e -output pad/naar/binary_bestand.arm64e` : pak een bestand met één architectuur van een universeel bestand uit

## Links
- More information: https://keith.github.io/xcode-man-pages/lipo.1.html
