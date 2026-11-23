# objdump

Bekijk informatie over objectbestanden.

## Usage
- `objdump -f pad/naar/binary` : toon de bestandsheaderinformatie
- `objdump -x pad/naar/binary` : toon alle headerinformatie
- `objdump -d pad/naar/binary` : toon de gedemonteerde uitvoer van uitvoerbare secties
- `objdump -d pad/naar/binary -M intel` : toon de gedemonteerde uitvoer van uitvoerbare secties in Intel-syntax
- `objdump -d pad/naar/binary --visualize-jumps=color|extended-color --disassembler-color=color|extended-color` : toon de gedemonteerde uitvoer van uitvoerbare secties met jump-visualisaties en syntax highlighting
- `objdump -t pad/naar/binary` : toon de symbooltabel
- `objdump -s pad/naar/binary` : toon een complete binary hex dump van alle secties

## Links
- More information: https://manned.org/objdump
