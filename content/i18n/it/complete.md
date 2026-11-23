# complete

Fornisce autocompletamento per argomenti dei comandi della shell.

## Usage
- `complete -F funzione comando` : applica ad un comando una funzione per gestirne l'autocompletamento
- `complete -C comando_per_autocompletamento comando` : applica ad un comando un altro comando per gestirne l'autocompletamento
- `complete -o nospace -F function comando` : applica l'autocompletamento senza aggiungere uno spazio dopo la parola completata

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-complete
