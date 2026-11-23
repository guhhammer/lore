# parallel

Voer commando's uit op meerdere CPU-kernen.

## Usage
- `parallel gzip ::: pad/naar/bestand1 pad/naar/bestand2 ...` : gzip meerdere bestanden tegelijk, met gebruik van alle kernen
- `ls *.txt | parallel -j 4 gzip` : lees argumenten van `stdin`, voer 4 taken tegelijk uit
- `parallel convert {} {.}.png ::: *.jpg` : converteer JPEG-afbeeldingen naar PNG met behulp van vervangingsstrings
- `argumenten | parallel -X commando` : werk zoals `xargs`, maar dan parallel, en stop zoveel mogelijk argumenten in één commando
- `cat groot_bestand.txt | parallel --pipe --block 1M commando` : verdeel `stdin` in blokken van ~1M, geef elk blok door aan `stdin` van een nieuw commando
- `parallel -S machine1,machine2 commando ::: arg1 arg2` : voer uit op meerdere machines via SSH
- `parallel -j 4 --bar --eta curl -sO {} :::: pad/naar/links.txt` : download 4 bestanden tegelijk vanuit een tekstbestand met links, met voortgangsweergave
- `parallel -t commando ::: argumenten` : toon de taken die `parallel` uitvoert in `stderr`

## Links
- More information: https://www.gnu.org/software/parallel/man.html
