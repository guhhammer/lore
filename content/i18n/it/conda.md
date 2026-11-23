# conda

Gestione pacchetti, dipendenze ed ambiente per qualsiasi linguaggio di programmazione. Alcuni comandi aggiuntivi, come `create`, hanno la propria documentazione.

## Usage
- `conda create -n nome_ambiente python=3.9 matplotlib` : crea un nuovo ambiente, installandovi alcuni pacchetti
- `conda info -e` : elenca tutti gli ambienti
- `conda activate nome_ambiente` : attiva un ambiente
- `conda deactivate` : disattiva un ambiente
- `conda remove -n nome_ambiente --all` : elimina un ambiente rimuovendo anche tutti i pacchetti
- `conda install python=3.4 numpy` : installa pacchetti nell'ambiente corrente
- `conda list` : elenca i pacchetti attualmente installati nell'ambiente corrente
- `conda clean -a` : elimina pacchetti inutilizzati e cache

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/index.html
