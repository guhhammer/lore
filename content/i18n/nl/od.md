# od

Toon bestandsinhoud in octale, decimale of hexadecimale notatie. Toon optioneel de byte-offsets en/of de afdrukbare weergave voor elke regel.

## Usage
- `od pad/naar/bestand` : toon bestand met de standaardinstellingen: octale notatie, 8 bytes per regel, byte-offsets in octale notatie en dubbele regels vervangen door `*`
- `od -v pad/naar/bestand` : toon bestand in uitgebreide modus, d.w.z. zonder dubbele regels te vervangen door `*`
- `od -t x -A d -v pad/naar/bestand` : toon bestand in hexadecimale notatie (2-byte eenheden), met byte-offsets in decimale notatie
- `od -t x1 -w4 -v pad/naar/bestand` : toon bestand in hexadecimale notatie (1-byte eenheden) en 4 bytes per regel
- `od -t xz -A n -v pad/naar/bestand` : toon bestand in hexadecimale notatie samen met de tekenweergave, en toon geen byte-offsets
- `od -N 100 -j 500 -v pad/naar/bestand` : lees slechts 100 bytes van een bestand vanaf de 500ste byte

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/od-invocation.html
