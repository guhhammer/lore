# conda create

Erstelle neue Conda-Umgebungen.

## Usage
- `conda create --yes --name py39 python=3.9 "numpy>=1.11"` : erstelle eine neue Umgebung mit dem Namen `py39` und installiere Python 3.9 und NumPy v1.11 (oder höher) darin
- `conda create --clone py39 --name py39-copy` : erstelle eine exakte Kopie einer Umgebung
- `conda create --name umgebungsname paketname` : erstelle eine neue Umgebung mit gegebenem Namen und den zu installierenden Paketen

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/create.html
