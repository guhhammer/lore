# diff

Confronta file e directory.

## Usage
- `diff file_vecchio file_nuovo` : confronta file (elenca i cambiamenti per trasformare `file_vecchio` in `file_nuovo`)
- `diff -w file_vecchio file_nuovo` : confronta file, ignorando gli spazi bianchi
- `diff -y file_vecchio file_nuovo` : confronta file, mostrando le differenze affiancate
- `diff -u file_vecchio file_nuovo` : confronta file, mostrando le differenze in formato unificato (come usato da `git diff`)
- `diff -r old_directory new_directory` : confronta directory ricorsivamente (mostra i nomi dei file/directory diversi e le modifiche ai file)
- `diff -r -q old_directory new_directory` : confronta directory, mostrando solo i nomi dei file diversi
- `diff -a -u -N file_vecchio file_nuovo >diff.patch` : crea un file patch per Git dalle differenze di due file di testo, trattando i file inesistenti come vuoti
- `diff -d --color=always file_vecchio file_nuovo` : confronta file, mostrando l'output colorato e cercando duramente un insieme minimo di cambiamenti

## Links
- More information: https://manned.org/diff
