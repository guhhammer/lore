# du

Schijfgebruik: schat en vat ruimtegebruik van bestanden en mappen samen.

## Usage
- `du -b|k|m pad/naar/map` : toon de grootte van een map en mogelijke submappen, met een gegeven eenheid (B/KiB/MiB)
- `du -h pad/naar/map` : toon de grootte van een map en mogelijke submappen, met een leesbaar formaat (d.w.z. door het automatisch kiezen van een eenheid gebaseerd op grootte)
- `du -sh pad/naar/map` : toon de grootte van een enkele map, in leesbare eenheden
- `du -ah pad/naar/map` : toon de grootte in leesbare vorm van een map met alle bestanden en mappen
- `du -h -d n pad/naar/map` : toon de grootte in leesbare vorm van een map en alle submappen tot `n` niveaus diep
- `du -ch *.jpg` : toon de grootte in leesbare vorm van alle `.jpg` bestanden in de huidige map en laat een cumulatief totaal zien op het eind
- `du -ah -t 1G|1024M|1048576K .[^.]* *` : toon alle bestanden en mappen (inclusief verborgen) boven een bepaalde drempelwaarde (bruikbaar om te onderzoeken wat veel ruimte inneemt)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/du-invocation.html
