# dconf write

Scrive i valori delle chiavi nei database dconf.

## Usage
- `dconf write /path/to/key "valore"` : scrive il valore di una chiave specifica
- `dconf write /path/to/key "'string'"` : scrive il valore di una chiave stringa specifica
- `dconf write /path/to/key "5"` : scrive il valore di una chiave intera specifica
- `dconf write /path/to/key "true|false"` : scrive il valore di una chiave booleana specifica
- `dconf write /path/to/key "['first', 'second', ...]"` : scrive il valore di una chiave array specifica
- `dconf write /path/to/key "@as []"` : scrive il valore di una chiave array vuota specifica

## Links
- More information: https://manned.org/dconf
