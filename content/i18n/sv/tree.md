# tree

Visa innehållet i den nuvarande katalogen som ett träd.

## Usage
- `tree -L num` : visa filer och katalogen upp till `num` nivåer djupt (där 1 är den nuvarande katalogen)
- `tree -d` : visa endast kataloger
- `tree -a -C` : visa även dolda filer med färgläggning på
- `tree -i -f` : visa trädstrukturen utan indenteringslinjer, med den fullständiga sökvägen till filen istället (använd `-N` för att inte maskera icke-utskrivbara tecken)
- `tree -s -h --du` : visa storleken för varje fil samt den sammanlagda storleken för varje katalog, i ett lättläst format
- `tree -P '*.txt' --prune` : visa filer i trädstrukturen som matchar ett jokerteckenmönster (`glob`), och uteslut kataloger som inte innehåller några matchande filer
- `tree -P katalog_namn --matchdirs --prune` : visa kataloger i trädstrukturen som matchar ett jokerteckenmönster (`glob`), och uteslut kataloger som inte ligger i sökvägen till den önskade katalogen
- `tree -I 'katalog_namn1|katalog_namn2'` : visa trädstrukturen utan de angivna katalogerna

## Links
- More information: https://manned.org/tree
