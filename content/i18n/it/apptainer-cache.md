# apptainer cache

Gestisce la cache locale di Apptainer.

## Usage
- `apptainer cache list` : elenca tutte le immagini container memorizzate nella cache
- `apptainer cache list -v` : elenca le immagini container memorizzate nella cache con informazioni dettagliate
- `apptainer cache list -T library|oci|shub|blob|...` : elenca solo un tipo specifico di cache
- `apptainer cache clean` : pulisce l'intera cache
- `apptainer cache clean -T library|oci|shub|blob|...` : pulisce solo un tipo specifico di cache
- `apptainer cache clean -D giorni` : pulisce le voci della cache più vecchie di un numero specifico di giorni
- `apptainer cache clean -n` : mostra un'anteprima di cosa verrebbe pulito senza rimuovere nulla
- `apptainer cache clean -f` : forza la pulizia senza conferma

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_cache.html
