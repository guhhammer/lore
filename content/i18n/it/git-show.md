# git show

Mostra vari tipi di oggetti Git (commit, tag, etc.).

## Usage
- `git show` : mostra informazioni sull'ultimo commit (hash, messaggio, modifiche, ed altri metadati)
- `git show commit` : mostra informazioni su un dato commit
- `git show tag` : mostra informazioni sul commit associato ad un tag specifico
- `git show ramo~3` : mostra informazioni sul terzo commit dalla cima del ramo
- `git show --oneline -s commit` : mostra il messaggio di commit su linea singola, senza mostrare il diff
- `git show --stat commit` : mostra solo la lista dei file modificati in un commit
- `git show revisione:percorso/del/file` : mostra il contenuto di un file ad una data revisione (ad esempio, in un ramo, tag o commit)

## Links
- More information: https://git-scm.com/docs/git-show
