# colordiff

Un'utilità per aggiungere colore all'output diff. Colordiff è un wrapper scritto in Perl per `diff` e produce lo stesso output, ma con una bella evidenziazione della sintassi. I colori possono essere personalizzati.

## Usage
- `colordiff file1 file2` : analisi di due file
- `colordiff -y file1 file2` : output in due colonne
- `colordiff -i file1 file2` : ignora differenze di maiuscole in file
- `colordiff -s file1 file2` : notifica se file identici
- `colordiff -w file1 file2` : ignora spazio vuoto (white space)

## Links
- More information: https://manned.org/colordiff
