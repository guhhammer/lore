# cut

Snij velden eruit vanuit `stdin` of bestanden.

## Usage
- `commando | cut -c 5` : toon het vijfde teken van elke regel
- `cut -c 5-10 pad/naar/bestand` : toon het vijfde tot en met tiende teken van elke regel van het opgegeven bestand
- `cut -f 2,6 pad/naar/bestand` : splits elke regel van een bestand op basis van een scheidingsteken in velden en toon velden twee en zes (standaard scheidingsteken is `TAB`)
- `commando | cut -d "scheidingsteken" -f 2-` : splits elke regel met het opgegeven scheidingsteken en toon alles vanaf het tweede veld
- `commando | cut -d " " -f -3` : gebruik een spatie als scheidingsteken en toon alleen de eerste drie velden
- `commando | cut -d ":" -f 1 -s` : toon alleen regels die het scheidingsteken bevatten
- `find . -print0 | cut -z -d "/" -f 2` : toon specifieke velden van regels die `NUL` gebruiken om regels af te sluiten in plaats van newlines

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cut-invocation.html
