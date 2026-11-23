# git reset

Annulla commit o rimuovi modifiche dall'area di stage, reimpostando l'HEAD corrente su uno specifico stato. Se viene fornito un percorso, il comando reset si interpreta come "rimuovi dall'area di stage"; se viene fornito l'hash di un commit o un ramo, si interpreta come "annulla commit".

## Usage
- `git reset` : rimuovi tutto dall'area di stage
- `git reset percorso/del/file1 percorso/del/file2 ...` : rimuovi dall'area di stage uno o più file
- `git reset -p percorso/del/file` : rimuovi dall'area di stage solo alcune porzioni di un file in modo interattivo
- `git reset HEAD~` : annulla l'ultimo commit, preservando tutte le modifiche nel filesystem
- `git reset --soft HEAD~2` : annulla gli ultimi due commit, aggiungendo all'area di stage le modifiche relative
- `git reset --hard` : annulla le modifiche non committate, indipendentemente se siano presenti nell'area di stage o meno (usa `git checkout` per queste ultime)
- `git reset --hard commit` : reimposta il repository su un dato commit, annullando qualsiasi tipo di modifica precedente

## Links
- More information: https://git-scm.com/docs/git-reset
