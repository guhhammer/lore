# conda env

Gestisce gli ambienti conda.

## Usage
- `conda env create -f percorso/del/file` : crea un ambiente da un file di ambiente (YAML, TXT, ecc.)
- `conda env remove -n nome_ambiente` : elimina un ambiente e tutto il suo contenuto
- `conda env update -f percorso/del/file --prune` : aggiorna un ambiente basandosi su un file di ambiente
- `conda env list` : elenca tutti gli ambienti
- `conda env export` : visualizza i dettagli dell'ambiente
- `conda env config vars list` : elenca le variabili d'ambiente
- `conda env config vars set mia_var=valore` : imposta variabili d'ambiente

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/env/index.html
