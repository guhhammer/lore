# sha512sum

Bereken SHA512 cryptografische checksums.

## Usage
- `sha512sum pad/naar/bestand1 pad/naar/bestand2 ...` : bereken de SHA512 checksum voor één of meer bestanden
- `sha512sum pad/naar/bestand1 pad/naar/bestand2 ... >pad/naar/bestand.sha512` : bereken en sla de lijst van SHA512 checksums op in een bestand
- `commando | sha512sum` : bereken een SHA512 checksum van `stdin`
- `sha512sum -c pad/naar/bestand.sha512` : lees een bestand met SHA512 checksums en bestandsnamen en verifieer dat alle bestanden overeenkomende checksums hebben
- `sha512sum -c --quiet pad/naar/bestand.sha512` : toon alleen een bericht voor ontbrekende bestanden of wanneer verificatie mislukt
- `sha512sum --ignore-missing -c --quiet pad/naar/bestand.sha512` : toon alleen een bericht wanneer verificatie mislukt, negeer ontbrekende bestanden
- `echo bekende_sha512_checksum_van_het_bestand pad/naar/bestand | sha512sum -c` : controleer een bekende SHA512 checksum van een bestand

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sha2-utilities.html
