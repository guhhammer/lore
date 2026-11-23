# hexdump

Een ASCII, decimale, hexadecimale, octale dump.

## Usage
- `hexdump pad/naar/bestand` : druk de hexadecimale weergave van een bestand af, waarbij dubbele regels worden vervangen door '*'
- `hexdump -C pad/naar/bestand` : toon de invoeroffset in hexadecimaal en zijn ASCII-representatie in twee kolommen
- `hexdump -C -n aantal_bytes pad/naar/bestand` : geef de hexadecimale weergave van een bestand weer, maar interpreteer alleen n bytes van de invoer
- `hexdump -v pad/naar/bestand` : vervang dubbele regels niet door '*'

## Links
- More information: https://manned.org/hexdump
