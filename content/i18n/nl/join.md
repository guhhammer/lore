# join

Voeg regels van twee gesorteerde bestanden samen op een gemeenschappelijk veld.

## Usage
- `join pad/naar/bestand1 pad/naar/bestand2` : voeg twee bestanden samen op het eerste (standaard) veld
- `join -t ',' pad/naar/bestand1 pad/naar/bestand2` : voeg twee bestanden samen met een komma (in plaats van een spatie) als veldscheidingsteken
- `join -1 3 -2 1 pad/naar/bestand1 pad/naar/bestand2` : voeg veld 3 van bestand 1 samen met veld 1 van bestand 2
- `join -a 1 pad/naar/bestand1 pad/naar/bestand2` : produceer een regel voor elke niet-koppelbare regel van bestand 1
- `cat pad/naar/bestand1 | join - pad/naar/bestand2` : voeg een bestand samen vanaf `stdin`

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/join-invocation.html
