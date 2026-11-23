# dconf

Gestisce i database dconf.

## Usage
- `dconf read /path/to/key` : stampa il valore di una chiave specifica
- `dconf list /percorso/della/directory/` : stampa le sottodirectory e le sotto-chiavi di un percorso specifico
- `dconf write /path/to/key "valore"` : scrive il valore di una chiave specifica
- `dconf reset /path/to/key` : reimposta il valore di una chiave specifica
- `dconf watch /path/to/key|/path/to/directory/` : osserva una chiave/directory specifica per rilevare modifiche
- `dconf dump /percorso/della/directory/` : mostra una directory specifica in formato file INI

## Links
- More information: https://manned.org/dconf
