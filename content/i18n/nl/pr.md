# pr

Pagineer of kolommeer bestanden voor afdrukken.

## Usage
- `pr pad/naar/bestand1 pad/naar/bestand2 ...` : toon meerdere bestanden met een standaardkop- en voettekst
- `pr -h "koptekst" pad/naar/bestand1 pad/naar/bestand2 ...` : toon met een aangepaste gecentreerde koptekst
- `pr -n -D "formaat" pad/naar/bestand1 pad/naar/bestand2 ...` : toon met genummerde regels en een aangepast datumnotatieformaat
- `pr -m -T pad/naar/bestand1 pad/naar/bestand2 ...` : toon alle bestanden samen, één in elke kolom, zonder kop- of voettekst
- `pr +2:5 -l paginalengte pad/naar/bestand1 pad/naar/bestand2 ...` : toon, beginnend bij pagina 2 en tot en met pagina 5, met een gegeven paginalengte (inclusief kop- en voettekst)
- `pr -o offset -W breedte pad/naar/bestand1 pad/naar/bestand2 ...` : toon met een offset voor elke regel en een afkappende aangepaste paginabreedte

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/pr-invocation.html
