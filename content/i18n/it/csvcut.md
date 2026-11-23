# csvcut

Filtra e tronca file CSV. Come il comando Unix `cut`, ma per dati tabellari. Incluso in csvkit.

## Usage
- `csvcut -n dati.csv` : stampa indici e nomi di tutte le colonne
- `csvcut -c 1,3 dati.csv` : estrai la prima e terza colonna
- `csvcut -C 4 dati.csv` : estrai tutte le colonne eccetto la quarta
- `csvcut -c id,"nome di battesimo" dati.csv` : estrai le colonne "id" e "nome di battesimo" (in quest'ordine)

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvcut.html
