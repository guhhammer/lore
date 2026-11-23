# black

Un formattatore automatico di codice Python.

## Usage
- `black percorso/del/file_o_directory` : auto-formatta un file o un'intera directory
- `black -c "codice"` : formatta il codice che gli viene passato come stringa
- `black --check percorso/del/file_o_directory` : verifica se i file necessitano di auto-formattazione senza modificare nulla
- `black --diff percorso/del/file_o_directory` : mostra i cambiamenti che verrebbero applicati a ciascun file
- `black -q percorso/del/file_o_directory` : auto-formatta un file o una directory senza produrre output
- `black -S percorso/del/file_o_directory` : auto-formatta un file o una directory senza sostituire gli apici con le doppie virgolette
