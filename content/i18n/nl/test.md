# test

Controleer bestandstypen en vergelijk waarden. Retourneert 0 als de voorwaarde waar is, 1 als de voorwaarde onwaar is.

## Usage
- `test "$MY_VAR" = "/bin/zsh"` : test of een gegeven variabele gelijk is aan een gegeven string
- `test -z "$GIT_BRANCH"` : test of een gegeven variabele een lege (z) waarde heeft
- `test -f "pad/naar/bestand"` : test of een bestand (f) bestaat
- `test ! -d "pad/naar/map"` : test of een map (d) niet bestaat
- `test voorwaarde && echo "true" || echo "false"` : als A waar is, voer dan B uit, of C in het geval van een fout (let op dat C mogelijk wordt uitgevoerd, zelfs als A mislukt)
- `if test -f "pad/naar/bestand"; then echo "File exists"; else echo "File does not exist"; fi` : gebruik `test` in een conditioneel statement
- `help test` : toon de help

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/test-invocation.html
