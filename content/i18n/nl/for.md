# for

Voer een commando meerdere keren uit.

## Usage
- `for variabele; do echo $variabele; done` : itereer over de commandoregelargumenten
- `for variabele in item1 item2 ...; do echo "Loop wordt uitgevoerd"; done` : voer de gegeven commando's uit voor elk van de opgegeven items
- `for variabele in {van..tot..stap}; do echo "Loop wordt uitgevoerd"; done` : itereer over een gegeven reeks nummers
- `for variabele in pad/naar/bestand1 pad/naar/bestand2 ...; do echo "Loop wordt uitgevoerd"; done` : itereer over een gegeven lijst van bestanden
- `for variabele in pad/naar/map1/ pad/naar/map2/ ...; do echo "Loop wordt uitgevoerd"; done` : itereer over een gegeven lijst van mappen
- `for variabele in */; do (cd "$variabele" || continue; echo "Loop wordt uitgevoerd") done` : voer een gegeven commando uit in elke map
- `for ((i=0; i<10; i++)); do echo $i; done` : gebruik een `for`-loop in C-stijl

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Looping-Constructs
