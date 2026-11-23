# csvstat

Stampa statistiche descrittive per tutte le colonne di un file CSV. Incluso in csvkit.

## Usage
- `csvstat dati.csv` : mostra tutte le statistiche per tutte le colonne
- `csvstat -c 2,4 dati.csv` : mostra tutte le statistiche per le colonne 2 e 4
- `csvstat --sum dati.csv` : mostra la somma per tutte le colonne
- `csvstat -c 3 --len dati.csv` : mostra la lunghezza massima dei valori della colonna 3
- `csvstat -c nome --unique dati.csv` : mostra il numero di valori unici nella colonna "nome"

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvstat.html
