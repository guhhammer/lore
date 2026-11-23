# airshare

Gegevens overdragen tussen twee machines in een lokaal netwerk.

## Usage
- `airshare code pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : deel bestanden of mappen
- `airshare code` : ontvang een bestand
- `airshare --upload code` : host een ontvangende server (gebruik deze om bestanden te kunnen uploaden via de webinterface)
- `airshare --upload code pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : stuur bestanden of mappen naar een ontvangende server
- `airshare --file-path code` : stuur bestanden waarvan de paden naar het klembord zijn gekopieerd
- `airshare --clip-receive code` : ontvang een bestand en kopieer het naar het klembord

## Links
- More information: https://airshare.readthedocs.io/en/latest/cli.html
