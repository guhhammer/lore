# conda export

Esporta i dettagli dell'ambiente.

## Usage
- `conda export` : esporta i dettagli dell'ambiente corrente su `stdout`
- `conda export -f percorso/a/environment.yaml` : esporta i dettagli dell'ambiente corrente in un file `YAML`
- `conda export --format environment-json|environment-yaml|explicit|json|reqs|requirements|txt|yaml|yml` : esporta i dettagli in un formato specifico
- `conda export -n nome_ambiente` : seleziona un ambiente per nome
- `conda export -p percorso/a/ambiente` : seleziona un ambiente tramite il suo percorso
- `conda export -c nome_canale` : includi un canale specifico

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/export.html
