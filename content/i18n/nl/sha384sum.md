# sha384sum

Bereken SHA384 cryptografische checksums.

## Usage
- `sha384sum pad/naar/bestand1 pad/naar/bestand2 ...` : bereken de SHA384 checksum voor één of meer bestanden
- `sha384sum pad/naar/bestand1 pad/naar/bestand2 ... >pad/naar/bestand.sha384` : bereken en sla de lijst van SHA384 checksums op in een bestand
- `commando | sha384sum` : bereken een SHA384 checksum van `stdin`
- `sha384sum -c pad/naar/bestand.sha384` : lees een bestand met SHA384 checksums en bestandsnamen en verifieer dat alle bestanden overeenkomende checksums hebben
- `sha384sum -c --quiet pad/naar/bestand.sha384` : toon alleen een bericht voor ontbrekende bestanden of wanneer verificatie mislukt
- `sha384sum --ignore-missing -c --quiet pad/naar/bestand.sha384` : toon alleen een bericht wanneer verificatie mislukt, negeer ontbrekende bestanden
- `echo bekende_sha384_checksum_van_het_bestand pad/naar/bestand | sha384sum -c` : controleer een bekende SHA384 checksum van een bestand

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sha2-utilities.html
