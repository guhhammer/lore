# ruff check

Een extreem snelle Python linter. `check` is het standaard commando - het kan overal weggelaten worden. Als geen bestanden of mappen zijn gespecificeerd, wordt standaard de huidige map gebruikt.

## Usage
- `ruff check pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : voer de linter uit op de opgegeven bestanden of mappen
- `ruff check --fix` : voer de gesuggereerde fixes uit en pas de bestanden in-place aan
- `ruff check --watch` : voer de linter uit en re-lint op iedere wijziging
- `ruff check --select ALL|regelcode1,regelcode2,...` : zet alleen de gespecificeerde regels (of alle regels) aan en negeer het configuratiebestand
- `ruff check --extend-select regelcode1,regelcode2,...` : zet additioneel de gespecificeerde regels aan
- `ruff check --ignore regelcode1,regelcode2,...` : zet de gespecificeerde regels uit
- `ruff check --select regelcode --add-noqa` : negeer alle bestaande schendingen van een regel door `# noqa` toe te voegen aan alle regels die de regel breken

## Links
- More information: https://docs.astral.sh/ruff/linter/
