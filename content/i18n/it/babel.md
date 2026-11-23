# babel

Un transpiler che converte codice JavaScript da sintassi ES6/ES7 ad ES5.

## Usage
- `babel percorso/del/file` : transpila uno specifico file e stampa il risultato su `stdout`
- `babel percorso/del/file_input --out-file percorso/del/file_output` : transpila un file e scrivi il risultato su uno specifico file di output
- `babel percorso/del/file --watch` : transpila un file ogni volta che viene modificato
- `babel percorso/della/directory_input` : transpila un'intera directory di file
- `babel percorso/della/directory_input --ignore file_ignorati` : transpila un'intera directory ignorando specifici file separati da virgola
- `babel percorso/del/file_input --minified` : transpila minimizzando il codice JavaScript in output
- `babel percorso/del/file_input --presets preset` : scegli un insieme di preset per formattare l'output
- `babel --help` : mostra l'aiuto

## Links
- More information: https://babeljs.io/docs/babel-cli
