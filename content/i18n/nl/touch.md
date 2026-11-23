# touch

Maak bestanden aan en stel toegang-/wijzigingstijden in.

## Usage
- `touch pad/naar/bestand1 pad/naar/bestand2 ...` : maak specifieke bestanden aan of werk hun tijdstempels bij als ze al bestaan
- `touch -c -a|-m pad/naar/bestand1 pad/naar/bestand2 ...` : stel de toegang- of wijzigingstijden (m) van een bestand in op de huidige tijd en maak (c) geen bestand aan als deze niet bestaat
- `touch -c -t YYYYMMDDHHMM.SS pad/naar/bestand1 pad/naar/bestand2 ...` : stel de tijd van een bestand in op een specifieke waarde en maak (c) geen bestand aan als deze niet bestaat
- `touch -c -r pad/naar/referentiebestand pad/naar/bestand1 pad/naar/bestand2 ...` : stel de timestamp van de bestanden in op die van het referentiebestand en maak (c) geen bestand aan als deze niet bestaat
- `touch -d "last year|5 hours|next thursday|nov 14|..." pad/naar/bestand` : stel de timestamp in door een string te parsen
- `touch pad/naar/bestand{1..10}` : maak meerdere bestanden met oplopende nummers
- `touch pad/naar/bestand{a..z}` : maak meerdere bestanden met een letterbereik
