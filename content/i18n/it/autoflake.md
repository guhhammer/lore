# autoflake

Uno strumento per rimuovere import e variabili inutilizzati da codice Python.

## Usage
- `autoflake --remove-unused-variables percorso/del/file.py` : rimuovi le variabili inutilizzate da un file e mostra la differenza
- `autoflake --remove-all-unused-imports percorso/del/file1.py percorso/del/file2.py ...` : rimuovi gli import inutilizzati da multipli file mostrando le differenze
- `autoflake --remove-unused-variables --in-place percorso/del/file.py` : rimuovi le variabili inutilizzate da un file, sovrascrivendolo
- `autoflake --remove-unused-variables --in-place --recursive percorso/della/directory` : rimuovi le variabili inutilizzate da tutti i file in una directory, ricorsivamente e sovrascrivendoli

## Links
- More information: https://github.com/PyCQA/autoflake#advanced-usage
