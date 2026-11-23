# conda create

Crea nuovi ambienti conda.

## Usage
- `conda create -y -n py39 python=3.9 "numpy>=1.11" scipy` : crea un nuovo ambiente chiamato `py39`, installa Python 3.9, NumPy v1.11 o superiore e l'ultima versione stabile di SciPy. Conferma automaticamente tutte le richieste
- `conda create -n myenv --file file1.yml --file file2.yml` : crea un nuovo ambiente chiamato `myenv` e installa i pacchetti elencati nei file specificati
- `conda create -p percorso/al/myenv` : crea un nuovo ambiente in un percorso personalizzato (prefisso)
- `conda create --clone py39 -n py39-copy` : crea una copia esatta di un ambiente chiamato `py39`
- `conda create -h` : mostra l'aiuto

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/create.html
