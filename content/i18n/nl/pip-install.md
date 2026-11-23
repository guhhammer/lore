# pip install

Installeer Python-pakketten.

## Usage
- `pip install pakket1 pakket2 ...` : installeer één of meer pakketten
- `pip install pakket1 pakket2 ... -U` : upgrade alle opgegeven pakketten naar de nieuwste versie en installeer alle pakketten die nog niet aanwezig zijn
- `pip install pakket==versie` : installeer een specifieke versie van een pakket
- `pip install -r pad/naar/requirements.txt` : installeer pakketten die in een bestand staan
- `pip install pad/naar/bestand.whl|pad/naar/bestand.tar.gz|pad/naar/map` : installeer een pakket van een lokaal archief of map
- `pip install git+https://example.com/gebruiker/repository.git` : installeer een pakket van een Git-repository
- `pip install -f url|pad/naar/map pakket` : installeer een pakket van een alternatieve bron (URL of map) in plaats van PyPI
- `pip install -e .` : installeer het lokale pakket in de huidige map in develop-modus (editable)

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_install/
