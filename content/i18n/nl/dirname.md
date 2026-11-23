# dirname

Verwijder het laatste bestandsnaamdeel van een pad.

## Usage
- `dirname pad/naar/bestand_of_map` : bereken de bovenliggende map van een opgegeven pad
- `dirname pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : bereken de bovenliggende map van meerdere paden
- `dirname -z pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : scheid de uitvoer met een NUL-teken in plaats van een nieuwe regel (handig bij gebruik met `xargs`)
