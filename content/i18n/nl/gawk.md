# gawk

GNU-versie van awk, een veelzijdige programmeertaal voor het werken met bestanden.

## Usage
- `gawk '{print $5}' pad/naar/bestand` : toon de vijfde kolom (a.k.a. veld) in een spatie-gescheiden bestand
- `gawk '/patroon/ {print $2}' pad/naar/bestand` : toon de tweede kolom van de regels die overeenkomen met een patroon in een spatie-gescheiden bestand
- `gawk -F ',' '{print $NF}' pad/naar/bestand` : toon de laatste kolom van elke regel in een bestand, met een komma (in plaats van spatie) als veldscheidingsteken
- `gawk '{s+=$1} END {print s}' pad/naar/bestand` : tel de waarden in de eerste kolom van een bestand op en toon het totaal
- `gawk 'NR%3==1' pad/naar/bestand` : toon elke derde regel, beginnend bij de eerste regel
- `gawk '{if ($1 == "string1") print "Exacte overeenkomst"; else if ($1 ~ "string2") print "Gedeeltelijke overeenkomst"; else print "Geen overeenkomst"}' pad/naar/bestand` : toon verschillende waarden op basis van voorwaarden
- `gawk '($10 >= min_waarde && $10 <= max_waarde)' pad/naar/bestand` : toon alle regels waarvan de waarde in de 10e kolom tussen een min en max ligt
- `gawk 'BEGIN {FS=":";printf "%-20s %6s %25s\n", "Name", "UID", "Shell"} $4 >= 1000 {printf "%-20s %6d %25s\n", $1, $4, $7}' /etc/passwd` : toon een tabel van gebruikers met UID >=1000 met koptekst en opgemaakte uitvoer, met een dubbele punt als scheidingsteken (`%-20s` betekent: 20 links uitgelijnde tekens, `%6s` betekent: 6 rechts uitgelijnde tekens)

## Links
- More information: https://www.gnu.org/software/gawk/manual/gawk.html
