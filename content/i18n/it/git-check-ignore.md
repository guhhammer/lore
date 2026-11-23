# git check-ignore

Analizza ed esegui il debug di `.gitignore` e dei file esclusi.

## Usage
- `git check-ignore percorso/del/file_o_directory` : verifica se un file o una directory sono ignorati
- `git check-ignore percorso/del/file_o_directory1 percorso/del/file_o_directory2 ...` : verifica se più file o directory sono ignorati
- `git <percorso/della/lista_dei_file_o_directory check-ignore --stdin` : leggi i percorsi di file o directory da `stdin` (uno per riga) invece che dalla riga di comando
- `git check-ignore --no-index percorso/del/file_o_directory1 percorso/del/file_o_directory2 ...` : non controllare nell'indice (usato per determinare il motivo per cui alcuni percorsi non sono ignorati)
- `git check-ignore -v percorso/del/file_o_directory1 percorso/del/file_o_directory2 ...` : includi dettagli sul pattern corrispondente per ogni percorso specificato

## Links
- More information: https://git-scm.com/docs/git-check-ignore
