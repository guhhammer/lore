# conda compare

Confronta i pacchetti tra ambienti conda.

## Usage
- `conda compare file.yml` : confronta i pacchetti nella directory corrente con quelli nel file `file.yml`
- `conda compare -n myenv percorso/a/file.yml` : confronta i pacchetti nell'ambiente chiamato `myenv` con quelli nel file `file.yml`
- `conda compare -p percorso/a/myenv percorso/a/file.yml` : confronta i pacchetti nell'ambiente `myenv` situato in un percorso personalizzato (prefisso) con quelli nel file `file.yml`
- `conda compare -h` : mostra l'aiuto

## Links
- More information: https://docs.conda.io/projects/conda/en/stable/commands/compare.html
