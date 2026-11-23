# difft

Confronta file o directory in base alla sintassi del linguaggio di programmazione.

## Usage
- `difft percorso/del/file_o_directory1 percorso/del/file_o_directory2` : confronta due file o directory
- `difft --check-only percorso/del/file1 percorso/del/file2` : segnala solo la presenza di differenze tra i file
- `difft --display side-by-side|side-by-side-show-both|inline|json percorso/del/file1 percorso/del/file2` : specifica la modalità di visualizzazione (default è `side-by-side`)
- `difft --ignore-comments percorso/del/file1 percorso/del/file2` : ignora i commenti durante il confronto
- `difft --syntax-highlight on|off percorso/del/file1 percorso/del/file2` : abilita/Disabilita la colorazione sintattica del codice sorgente (default è `on`)
- `difft --skip-unchanged percorso/del/file_o_directory1 percorso/del/file_o_directory2` : non outputta nulla se non ci sono differenze tra i file
- `difft --list-languages` : stampa tutti i linguaggi di programmazione supportati dallo strumento, con le loro estensioni

## Links
- More information: https://difftastic.wilfred.me.uk/introduction.html
