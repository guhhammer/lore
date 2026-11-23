# base64

Encodeer of decodeer een bestand of `stdin` van/naar Base64 naar `stdout`.

## Usage
- `base64 pad/naar/bestand` : encodeer een bestand
- `base64 -w 0|76|... pad/naar/bestand` : zet gecodeerde uitvoer naar een specifieke breedte (`0` schakelt het uit)
- `base64 -d pad/naar/bestand` : decodeer een bestand
- `commando | base64` : encodeer `stdin`
- `commando | base64 -d` : decodeer `stdin`

## Links
- More information: https://manned.org/base64
