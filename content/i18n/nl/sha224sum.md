# sha224sum

Bereken SHA224 cryptografische checksums.

## Usage
- `sha224sum pad/naar/bestand1 pad/naar/bestand2 ...` : bereken de SHA224 checksum voor één of meer bestanden
- `sha224sum pad/naar/bestand1 pad/naar/bestand2 ... >pad/naar/bestand.sha224` : bereken en sla de lijst van SHA224 checksums op in een bestand
- `commando | sha224sum` : bereken een SHA224 checksum van `stdin`
- `sha224sum -c pad/naar/bestand.sha224` : lees een bestand met SHA224 checksums en bestandsnamen en verifieer dat alle bestanden overeenkomende checksums hebben
- `sha224sum -c --quiet pad/naar/bestand.sha224` : toon alleen een bericht voor ontbrekende bestanden of wanneer verificatie mislukt
- `sha224sum --ignore-missing -c --quiet pad/naar/bestand.sha224` : toon alleen een bericht wanneer verificatie mislukt, negeer ontbrekende bestanden
- `echo bekende_sha224_checksum_van_het_bestand pad/naar/bestand | sha224sum -c` : controleer een bekende SHA224 checksum van een bestand

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sha2-utilities.html
