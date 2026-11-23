# du

Utilizzo del disco: stima e riassumi lo spazio utilizzato da file e directory.

## Usage
- `du -b|k|m percorso/della/directory` : elenca le dimensioni di una directory ed ogni sotto-directory, nell'unità specificata (B/KiB/MiB)
- `du -h percorso/della/directory` : elenca le dimensioni di una directory ed ogni sotto-directory, in formato umanamente leggibile (seleziona automaticamente l'unità appropriata per ogni dimensione)
- `du -sh percorso/della/directory` : mostra la dimensione di una singola directory, in unità umanamente leggibili
- `du -ah percorso/della/directory` : mostra in formato umanamente leggibile le dimensioni di una directory e tutti i file e directory in essa contenuti
- `du -h -d N percorso/della/directory` : elenca le dimensioni umanamente leggibili di una directory e d ogni sotto-directory, fino ad N livelli di profondità
- `du -ch */*.jpg` : mostra le dimensioni umanamente leggibili di tutti i file `.jpg` nelle sottodirectory della directory corrente, e mostra il totale cumulativo alla fine

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/du-invocation.html
