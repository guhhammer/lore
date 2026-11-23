# pio pkg

Beheer pakketten in het register. Pakketten kunnen alleen verwijderd worden binnen 72 uur (3 dagen) vanaf de datum dat ze gepubliceerd zijn.

## Usage
- `pio pkg pack -o pad/naar/pakket.tar.gz` : maak een pakket tarball van de huidige map
- `pio pkg publish` : maak en publiceer een pakket tarball van de huidige map
- `pio pkg publish --private` : publiceer de huidige map en beperk de publieke toegang
- `pio pkg publish pad/naar/pakket.tar.gz` : publiceer een pakket
- `pio pkg publish pad/naar/pakket.tar.gz --released-at "2021-04-08 21:15:38"` : publiceer een pakket met een aangepaste release datum (UTC)
- `pio pkg unpublish pakket` : verwijder alle versies van een gepubliceerd pakket van het register
- `pio pkg unpublish pakket@versie` : verwijder een specifieke versie van een gepubliceerd pakket van het register
- `pio pkg unpublish --undo pakket@versie` : maak de verwijdering ongedaan en zet alle versies of een specifieke versie van het pakket terug in het register

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/package/
