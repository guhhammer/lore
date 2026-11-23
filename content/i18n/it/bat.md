# bat

Stampa e concatena file. Un clone di `cat` con syntax highlighting e integrazione Git.

## Usage
- `bat percorso/del/file1 percorso/del/file2 ...` : stampa in modo formattato i contenuti di uno o più file su `stdout`
- `bat percorso/del/file1 percorso/del/file2 ... >percorso/del/file_destinazione` : concatena diversi file nel file di destinazione
- `bat -pp percorso/del/file` : rimuove decorazioni e disabilita paging
- `bat -H 10|5:10|:10|10:|10:+5 percorso/del/file` : evidenzia una riga specifica o un intervallo di righe con un colore di sfondo diverso
- `bat -A percorso/del/file` : mostra caratteri non stampabili come spazio, tab o newline
- `bat -n percorso/del/file` : rimuove tutte le decorazioni tranne i numeri di riga nell'output
- `bat -l json percorso/del/file.json` : evidenzia la sintassi di un file JSON impostando esplicitamente il linguaggio
- `bat -L` : visualizza tutti i linguaggi supportati

## Links
- More information: https://manned.org/bat
