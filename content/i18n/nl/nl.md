# nl

Voorzie regels van een nummer uit een bestand of van `stdin`.

## Usage
- `nl pad/naar/bestand` : voorzie niet-lege regels in een bestand van een nummer
- `commando | nl -` : lees van `stdin`
- `nl -b a|n pad/naar/bestand` : nummer alle body regels inclusief lege regels of nummer geen body regels
- `nl -b p'FooBar[0-9]' pad/naar/bestand` : nummer alleen de body regels die overeenkomen met een basis `regex` (BRE) patroon
- `nl -i increment pad/naar/bestand` : gebruik een specifieke increment voor regelnummering
- `nl -n rz|ln|rn` : specificeer het nummeringsformaat voor regels: rechts of links uitgelijnd, met voorloopnullen (zeros) of niet
- `nl -w kolombreedte pad/naar/bestand` : specificeer de breedte (w) van de nummering (standaard is 6)
- `nl -s scheidingsteken pad/naar/bestand` : gebruik een specifieke string om de regelnummers van de regels te scheiden (standaard is `TAB`)

## Links
- More information: https://manned.org/nl.1p
