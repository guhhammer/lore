# tree

Mostra i contenuti della directory corrente come un albero.

## Usage
- `tree -L num` : stampa file e directory fino al 'num'-esimo livello di profondità (dove 1 significa la directory corrente)
- `tree -d` : stampa solamente le directory
- `tree -a -C` : stampa anche i file nascosti con la colorazione attiva
- `tree -i -f` : stampa l'albero senza linee di indentazione, mostrando invece il percorso completo (usa `-N` per non convertire caratteri non stampabili in sequenze di escape)
- `tree -s -h --du` : stampa la dimensione di ogni file e la dimensione totale di ogni directory, in formato leggibile dall'utente
- `tree -P '*.txt' --prune` : stampa i file all'interno dell'albero gerarchico, utilizzando espressioni di metacaratteri (`glob` pattern) per escludere le directory che non contengono file corrispondenti alla ricerca
- `tree -P nomi_di_directory --matchdirs --prune` : stampa le directory all'interno dell'albero gerarchico, utilizzando espressioni di metacaratteri (`glob` pattern) per escludere le directory che non sono progenitori di quelle desiderate
- `tree -I 'nome_directory1|nome_directory2'` : stampa l'albero ignorando le directory date

## Links
- More information: https://manned.org/tree
