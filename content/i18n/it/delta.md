# delta

Visualizzatore per output Git e diff.

## Usage
- `delta percorso/del/vecchio_file_o_directory percorso/del/nuovo_file_o_directory` : confronta file o directory
- `delta -n percorso/del/vecchio_file_o_directory percorso/del/nuovo_file_o_directory` : confronta file o directory, mostrando i numeri di riga
- `delta -s percorso/del/vecchio_file_o_directory percorso/del/nuovo_file_o_directory` : confronta file o directory, mostrando le differenze affiancate
- `delta --no-gitconfig percorso/del/vecchio_file_o_directory percorso/del/nuovo_file_o_directory` : confronta file o directory, ignorando le impostazioni di configurazione Git
- `delta --hyperlinks percorso/del/vecchio_file_o_directory percorso/del/nuovo_file_o_directory` : confronta, rendendo hash commit, nomi file e numeri di riga come collegamenti ipertestuali secondo la specifica per emulatori terminal
- `delta --show-config` : visualizza le impostazioni correnti
- `delta --list-languages` : visualizza i linguaggi supportati e le estensioni di file associate

## Links
- More information: https://dandavison.github.io/delta/full---help-output.html
