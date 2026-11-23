# awk

Een veelzijdige programmeertaal voor het werken met bestanden. Opmerking: Verschillende implementaties van AWK maken dit vaak een symlink van hun binary.

## Usage
- `awk '{print $5}' pad/naar/bestand` : toon de vijfde kolom (a.k.a. veld) in een spatie-gescheiden bestand
- `awk '/string1/ {print $2}' pad/naar/bestand` : toon de tweede kolom van de regels die "string1" bevatten in een spatie-gescheiden bestand
- `awk -F ',' '{print $NF}' pad/naar/bestand` : toon de laatste kolom van iedere regel in een bestand en maak gebruik van een komma (in plaats van een spatie) als veldscheider
- `awk '{s+=$1} END {print s}' pad/naar/bestand` : tel de waarden in de eerste kolom van een bestand op en toon het totaal
- `awk 'NR%3==1' pad/naar/bestand` : toon iedere derde regel startend vanaf de eerste regel
- `awk '{if ($1 == "string1") print "Exact match string1"; else if ($1 ~ "string2") print "Partial match string2"; else print "string3"}' pad/naar/bestand` : toon verschillende waardes gebaseerd op condities
- `awk '($10 >= min_waarde && $10 <= max_waarde)' pad/naar/bestand` : toon alle regels waarbij de waarde van de 10e kolom tussen een minimale en maximale waarde is
- `awk 'BEGIN {FS=":";printf "%-20s %6s %25s\n", "Name", "UID", "Shell"} $4 >= 1000 {printf "%-20s %6d %25s\n", $1, $4, $7}' /etc/passwd` : print een tabel van gebruikers met UID >= 1000 met header en opgemaakte uitvoer, gebruikmakend van een dubbele punt als scheidingsteken (`%-20s` betekent: 20 links uitgelijnde tekens, `%6s` betekent: 6 rechts uitgelijnde tekens)

## Links
- More information: https://github.com/onetrueawk/awk
