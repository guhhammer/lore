# seq

Toon een reeks getallen naar `stdout`.

## Usage
- `seq 10` : toon een reeks van 1 tot 10
- `seq 10 20` : toon een reeks van 10 tot 20
- `seq 5 3 20` : toon ieder 3e nummer in een reeks van 5 tot 20
- `seq 10 -1 1` : toon een omgekeerde reeks van 10 tot 1
- `seq -s " " 5 3 20` : scheid de uitvoer met een spatie in plaats van een nieuwe regel
- `seq -f "%04g" 5 3 20` : formatteer de uitvoerbreedte naar minimaal 4 cijfers, opgevuld met nullen indien nodig
- `seq -w 5 3 20` : toon alle nummers met dezelfde breedte

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/seq-invocation.html
