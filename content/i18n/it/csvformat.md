# csvformat

Converti un file CSV in un formato di output personalizzato. Incluso in csvkit.

## Usage
- `csvformat -T dati.csv` : converti in un file delimitato da tab (TSV)
- `csvformat -D "carattere_personalizzato" dati.csv` : converti i delimitatori in un carattere personalizzato
- `csvformat -M "\r\n" dati.csv` : converti caratteri newline a carriage return (^M) + newline
- `csvformat -U 0 dati.csv` : minimizza l'utilizzo delle virgolette
- `csvformat -U 1 dati.csv` : massimizza l'utilizzo delle virgolette

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvformat.html
