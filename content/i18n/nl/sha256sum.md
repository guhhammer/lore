# sha256sum

Bereken SHA256 cryptografische checksums.

## Usage
- `sha256sum pad/naar/bestand1 pad/naar/bestand2 ...` : bereken de SHA256 checksum voor één of meer bestanden
- `sha256sum pad/naar/bestand1 pad/naar/bestand2 ... >pad/naar/bestand.sha256` : bereken en sla de lijst van SHA256 checksums op in een bestand
- `commando | sha256sum` : bereken een SHA256 checksum van `stdin`
- `sha256sum -c pad/naar/bestand.sha256` : lees een bestand met SHA256 checksums en bestandsnamen en verifieer dat alle bestanden overeenkomende checksums hebben
- `sha256sum -c --quiet pad/naar/bestand.sha256` : toon alleen een bericht voor ontbrekende bestanden of wanneer verificatie mislukt
- `sha256sum --ignore-missing -c --quiet pad/naar/bestand.sha256` : toon alleen een bericht wanneer verificatie mislukt, negeer ontbrekende bestanden
- `echo bekende_sha256_checksum_van_het_bestand pad/naar/bestand | sha256sum -c` : controleer een bekende SHA256 checksum van een bestand

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sha2-utilities.html
