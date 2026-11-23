# tac

Visualizza e concatena file con righe in ordine inverso.

## Usage
- `tac percorso/del/file1 percorso/del/file2 ...` : concatena file specifici in ordine inverso
- `cat percorso/del/file | tac` : visualizza `stdin` in ordine inverso
- `tac -s separatore percorso/del/file1 percorso/del/file2 ...` : usa un sriparatore specifico
- `tac -r -s separatore percorso/del/file1 percorso/del/file2 ...` : usa un regex specifico come separatore
- `tac -b percorso/del/file1 percorso/del/file2 ...` : utilizzare un separatore bprima di ciascun file

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tac-invocation.html
