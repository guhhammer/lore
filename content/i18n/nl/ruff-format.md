# ruff format

Een extreem snelle Python code formatter. Als geen bestanden of mappen zijn gespecificeerd, wordt standaard de huidige map gebruikt.

## Usage
- `ruff format pad/naar/bestand_of_map1 pad/naar/bestand_of_map2 ...` : formatteer opgegeven bestanden of mappen in-place
- `ruff format --check` : toon welke bestanden aangepast zouden worden en return een niet-nul exit code als er bestanden zijn om te formatteren en anders nul
- `ruff format --diff` : toon welke veranderingen er gemaakt zouden worden zonder de bestanden aan te passen

## Links
- More information: https://docs.astral.sh/ruff/formatter/
