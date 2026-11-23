# sha1sum

Bereken SHA1 cryptografische checksums.

## Usage
- `sha1sum pad/naar/bestand1 pad/naar/bestand2 ...` : bereken de SHA1 checksum voor één of meer bestanden
- `sha1sum pad/naar/bestand1 pad/naar/bestand2 ... >pad/naar/bestand.sha1` : bereken en sla de lijst van SHA1 checksums op in een bestand
- `commando | sha1sum` : bereken een SHA1 checksum van `stdin`
- `sha1sum -c pad/naar/bestand.sha1` : lees een bestand met SHA1 checksums en bestandsnamen en verifieer dat alle bestanden overeenkomende checksums hebben
- `sha1sum -c --quiet pad/naar/bestand.sha1` : toon alleen een bericht voor ontbrekende bestanden of wanneer verificatie mislukt
- `sha1sum --ignore-missing -c --quiet pad/naar/bestand.sha1` : toon alleen een bericht wanneer verificatie mislukt, negeer ontbrekende bestanden
- `echo bekende_sha1_checksum_van_het_bestand pad/naar/bestand | sha1sum -c` : controleer een bekende SHA1 checksum van een bestand
