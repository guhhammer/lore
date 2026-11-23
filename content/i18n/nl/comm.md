# comm

Toon overeenkomstige regels tussen twee bestanden. Beide bestanden dienen gesorteerd te zijn.

## Usage
- `comm bestand1 bestand2` : produceer drie tab-gescheiden kolommen: regels die alleen voorkomen in het eerste bestand, regels die alleen voorkomen in het tweede bestand en overeenkomstige regels tussen beide bestanden
- `comm -12 bestand1 bestand2` : toon alleen overeenkomstige regels van beide bestanden
- `cat bestand1 | comm -12 - bestand2` : toon alleen de overeenkomstige regels van beide bestanden en lees een bestand vanaf `stdin`
- `comm -23 bestand1 bestand2 >alleen_bestand1` : sla regels die alleen in het eerste bestand worden gevonden op in een derde bestand
- `comm -13 <(sort bestand1) <(sort bestand2)` : toon de regels welke alleen in het tweede bestand gevonden worden, als de bestanden niet gesorteerd zijn

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/comm-invocation.html
