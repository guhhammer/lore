# base32

Encodeer of decodeer een bestand of `stdin` van/naar Base32 naar `stdout`.

## Usage
- `base32 pad/naar/bestand` : encodeer een bestand
- `base32 -w 0|76|... pad/naar/bestand` : zet gecodeerde uitvoer naar een specifieke breedte (`0` schakelt het uit)
- `base32 -d pad/naar/bestand` : decodeer een bestand
- `commando | base32` : encodeer `stdin`
- `commando | base32 -d` : decodeer `stdin`

## Links
- More information: https://manned.org/base32
