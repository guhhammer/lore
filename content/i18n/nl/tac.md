# tac

Toon en voeg bestanden samen met regels in omgekeerde volgorde.

## Usage
- `tac pad/naar/bestand1 pad/naar/bestand2 ...` : voeg specifieke bestanden samen in omgekeerde volgorde
- `cat pad/naar/bestand | tac` : toon `stdin` in omgekeerde volgorde
- `tac -s scheidingsteken pad/naar/bestand1 pad/naar/bestand2 ...` : gebruik een specifiek scheidingsteken
- `tac -r -s scheidingsteken pad/naar/bestand1 pad/naar/bestand2 ...` : gebruik een specifieke `regex` als scheidingsteken
- `tac -b pad/naar/bestand1 pad/naar/bestand2 ...` : gebruik een scheidingsteken vóór elk bestand

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tac-invocation.html
