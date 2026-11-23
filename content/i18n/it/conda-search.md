# conda search

Cerca pacchetti e mostra i loro dettagli.

## Usage
- `conda search nome_pacchetto` : cerca un pacchetto specifico
- `conda search nome_pacchetto -i` : cerca un pacchetto con i relativi dettagli
- `conda search "*stringa*"` : cerca pacchetti che contengono `stringa` nel nome
- `conda search "nome_pacchetto>=versione_pacchetto"` : cerca una versione specifica di un pacchetto
- `conda search canale::nome_pacchetto` : cerca un pacchetto all'interno di un canale specifico
- `conda search --envs nome_pacchetto` : verifica se un pacchetto è installato in uno qualsiasi degli ambienti locali

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/search.html
