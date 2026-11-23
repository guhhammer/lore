# uv add

Voeg pakketafhankelijkheden toe aan het `pyproject.toml` bestand.

## Usage
- `uv add pakket` : voeg de nieuwste versie van een pakket toe
- `uv add pakket1 pakket2 ...` : voeg meerdere pakketten toe
- `uv add pakket>=1.2.3` : voeg een pakket toe met een versievereiste
- `uv add --optional optioneel pakket1 pakket2 ...` : voeg pakketten toe aan een optionele afhankelijkheidsgroep, die wordt opgenomen wanneer ze worden gepubliceerd
- `uv add --group groep pakket1 pakket2 ...` : voeg pakketten toe aan een lokale groep, die niet wordt opgenomen wanneer ze worden gepubliceerd
- `uv add --dev pakket1 pakket2 ...` : voeg pakketten toe aan de dev-groep, afkorting voor `--group dev`
- `uv add --editable pad/naar/pakket/` : voeg pakket toe als bewerkbaar
- `uv add pakket --extra extra_functie` : schakel een extra in bij het installeren van een pakket, deze kan meerdere keren opgegeven worden

## Links
- More information: https://peps.python.org/pep-0508/
