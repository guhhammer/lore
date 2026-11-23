# b2sum

Bereken BLAKE2 cryptografische checksums.

## Usage
- `b2sum pad/naar/bestand1 pad/naar/bestand2 ...` : bereken de BLAKE2 checksum voor een of meerdere bestanden
- `b2sum pad/naar/bestand1 pad/naar/bestand2 ... >pad/naar/bestand.b2` : bereken en sla de lijst van BLAKE2 checksums op in een bestand
- `commando | b2sum` : bereken een BLAKE2 checksum van `stdin`
- `b2sum -c pad/naar/bestand.b2` : lees een bestand met BLAKE2 checksums en bestandsnamen en verifieer dat alle bestanden overeenkomende checksums hebben
- `b2sum -c --quiet pad/naar/bestand.b2` : toon alleen een melding voor missende bestanden of als verificatie faalt
- `b2sum --ignore-missing -c --quiet pad/naar/bestand.b2` : toon alleen een melding als een verificatie faalt en negeer missende bestanden
- `echo bekende_blake2_checksum_van_het_bestand pad/naar/bestand | b2sum -c` : controleer een bekende BLAKE2 checksum van een bestand
