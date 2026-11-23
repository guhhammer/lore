# enscript

Converti file di testo in PostScript, HTML, RTF, ANSI ed overstrike.

## Usage
- `enscript percorso/del/file_input -o percorso/del/file_output` : genera un file PostScript da un file di testo
- `enscript percorso/del/file_input -w linguaggio -o percorso/del/file_output` : genera un file in un linguaggio differente da PostScript
- `enscript percorso/del/file_input --columns numero_colonne -r -o percorso/del/file_output` : genera un file PostScript con layout orizzontale, dividendo la pagina in colonne (massimo 9)
- `enscript --help-highlight` : mostra linguaggi e formati file disponibili per evidenziare la sintassi
- `enscript percorso/del/file_input --color 1 -E linguaggio -o percorso/del/file_output` : genera un file PostScript con evidenziazione della sintassi e colori per uno specifico linguaggio

## Links
- More information: https://manned.org/enscript
