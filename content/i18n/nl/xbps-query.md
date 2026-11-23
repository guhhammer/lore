# xbps-query

XBPS hulpprogramma om te zoeken naar pakket- en repository-informatie.

## Usage
- `xbps-query -s regex|trefwoord --repository --regex` : zoek naar een pakket in externe repositories met behulp van een `regex` of een trefwoord (als `--regex` wordt weggelaten)
- `xbps-query -S pakket` : toon informatie over een geïnstalleerd pakket
- `xbps-query -S pakket --repository` : toon informatie over een pakket in externe repositories
- `xbps-query -l` : toon alle geregistreerde pakketten in de pakketdatabase
- `xbps-query -m` : toon expliciet geïnstalleerde pakketten (bijv. niet automatisch geïnstalleerd als afhankelijkheden)

## Links
- More information: https://manned.org/xbps-query
