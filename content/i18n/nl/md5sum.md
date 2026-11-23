# md5sum

Bereken MD5 cryptografische checksums.

## Usage
- `md5sum pad/naar/bestand1 pad/naar/bestand2 ...` : bereken de MD5 checksum voor één of meer bestanden
- `md5sum pad/naar/bestand1 pad/naar/bestand2 ... >pad/naar/bestand.md5` : bereken en sla de lijst van MD5 checksums op in een bestand
- `commando | md5sum` : bereken een MD5 checksum van `stdin`
- `md5sum -c pad/naar/bestand.md5` : lees een bestand met MD5 checksums en bestandsnamen en verifieer dat alle bestanden overeenkomende checksums hebben
- `md5sum -c --quiet pad/naar/bestand.md5` : toon alleen een bericht voor ontbrekende bestanden of wanneer verificatie mislukt
- `md5sum --ignore-missing -c --quiet pad/naar/bestand.md5` : toon alleen een bericht wanneer verificatie mislukt, negeer ontbrekende bestanden
- `echo bekende_md5_checksum_van_het_bestand pad/naar/bestand | md5sum -c` : controleer een bekende MD5 checksum van een bestand
