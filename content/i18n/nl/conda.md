# conda

Pakket-, afhankelijkheids- en omgevingsbeheer voor alle programmeertalen. Sommige subcommando's zoals `create` hebben hun eigen documentatie.

## Usage
- `conda create -n omgevingsnaam python=3.9 matplotlib` : maak een nieuwe omgeving aan en installeer hierin benoemde pakketten
- `conda info -e` : toon alle omgevingen
- `conda activate omgevingsnaam` : activeer een omgeving
- `conda deactivate` : deactiveer een omgeving
- `conda remove -n omgevingsnaam --all` : verwijder een omgeving (verwijder alle pakketten)
- `conda install python=3.4 numpy` : installeer pakketten in de huidige omgeving
- `conda list` : toon geïnstalleerde pakketten in de huidige omgeving
- `conda clean -a` : verwijder ongebruikte pakketten en caches

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/index.html
