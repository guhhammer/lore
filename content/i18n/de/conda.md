# conda

Eine Paket-, Abhängigkeits- und Umgebungsverwaltung für beliebige Programmiersprachen. Manche Unterbefehle wie `create` sind separat dokumentiert.

## Usage
- `conda create -n umgebungsname python=3.9 matplotlib` : erstelle eine neue Umgebung mit den zu installierenden Paketen
- `conda info -e` : liste alle Umgebungen auf
- `conda activate umgebungs_name` : lade eine Umgebung
- `conda deactivate` : entlade eine Umgebung
- `conda remove -n umgebungsname --all` : lösche eine Umgebung (entferne alle Pakete)
- `conda install python=3.4 numpy` : installiere Pakete in die derzeit geladene Umgebung
- `conda list` : liste alle installierten Pakete in der derzeit geladenen Umgebung auf
- `conda clean -a` : lösche alle ungenutzten Pakete und leere den Cache

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/index.html
