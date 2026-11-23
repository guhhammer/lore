# uv run

Voer een commando of script uit in de projectomgeving.

## Usage
- `uv run pad/naar/script.py` : voer een Python-script uit
- `uv run -m modulenaam` : voer een Python-module uit
- `uv run -w pakket commando` : voer een opdracht uit met tijdelijk geïnstalleerde extra pakketten
- `uv run --with-requirements pad/naar/requirements.txt pad/naar/script.py` : voer een script uit met pakketten van een requirements-bestand
- `uv run --isolated pad/naar/script.py` : voer uit in een geïsoleerde omgeving (geen projectafhankelijkheden)
- `uv run --no-sync commando` : voer uit zonder de omgeving eerst te synchroniseren

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-run
