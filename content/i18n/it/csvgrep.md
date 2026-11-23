# csvgrep

Filtra righe CSV con stringhe e pattern matching. Incluso in csvkit.

## Usage
- `csvgrep -c 1 -m stringa data.csv` : trova righe contenenti una certa stringa nella colonna 1
- `csvgrep -c 3,4 -r espressione_regolare data.csv` : trova righe per le quali le colonne 3 e 4 soddisfano una certa espressione regolare
- `csvgrep -i -c nome -m "Mario Rossi" data.csv` : trova righe dove la colonna "nome" NON include la stringa "Mario Rossi"

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvgrep.html
