# dirname

Determina la directory genitore di un determinato file o percorso.

## Usage
- `dirname percorso/del/file_o_directory` : calcola la directory genitore di un dato percorso
- `dirname percorso/del/file_o_directory1 percorso/del/file_o_directory2 ...` : calcola la directory genitore di più percorsi
- `dirname -z percorso/del/file_o_directory1 percorso/del/file_o_directory2 ...` : delimita l'output utilizzando caratteri NUL invece di una nuova linea (utile in combinazione con `xargs`)
