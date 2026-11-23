# git restore

Ripristina i file dell'albero di lavoro. Richiede versioni di Git successive alla 2.23.

## Usage
- `git restore percorso/del/file` : ripristina un file cancellato dal contenuto del commit corrente (HEAD)
- `git restore --source commit percorso/del/file` : ripristina un file alla versione di un commit differente
- `git restore .` : annulla le modifiche ai file nell'area di stage, ripristinandoli all'HEAD

## Links
- More information: https://git-scm.com/docs/git-restore
