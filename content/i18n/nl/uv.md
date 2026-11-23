# uv

Een snelle Python pakket- en projectbeheerder. Sommige subcommando's zoals `tool`, `python`, etc. hebben hun eigen documentatie.

## Usage
- `uv init` : creëer een nieuw Python project in de huidige map
- `uv init pad/naar/map` : creëer een nieuw Python project in het opgegeven pad
- `uv add pakket` : voeg een nieuwe afhankelijkheid toe aan het project
- `uv remove pakket` : verwijder een afhankelijkheid van het project
- `uv run pad/naar/script.py|commando` : voer een script of commando uit in de projectomgeving
- `uv sync` : update een projectomgeving vanuit `pyproject.toml`
- `uv lock` : creëer een lockbestand voor de afhankelijkheden van het project
- `uv build` : bouw het project in bron- en binary distributies

## Links
- More information: https://docs.astral.sh/uv/reference/cli/
