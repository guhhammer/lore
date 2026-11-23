# sdelete

Verwijder veilig een bestand/map van de schijf, of maak de vrije ruimte op een volume/fysieke schijf schoon.

## Usage
- `sdelete -p 3 pad\naar\bestand1 pad\naar\bestand2 ...` : verwijder bestanden met 3 passes
- `sdelete -s pad\naar\map1 pad\naar\map2 ...` : verwijder mappen en de submappen met 1 pass (default)
- `sdelete -p 3 D:` : maak de vrije ruimte schoon van volume D: met 3 passes
- `sdelete -z 2` : maak de vrije ruimte schoon met nullen (zeros) van fysieke schijf 2, welke geen volumes meer mag bevatten die opgeschoond kunnen worden

## Links
- More information: https://learn.microsoft.com/sysinternals/downloads/sdelete
