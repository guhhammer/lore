# git stash

Salva in un'area temporanea (stash) modifiche Git locali.

## Usage
- `git stash push -m messaggio_di_stash_opzionale` : salva in un'area temporanea modifiche locali, tranne i file nuovi e non tracciati
- `git stash -u` : includi nello stash anche i file nuovi e non tracciati
- `git stash -p` : seleziona per lo stash parti di file modificati in modo interattivo
- `git stash list` : elenca tutti gli stash, mostrandone il nome, ramo relativo e messaggio
- `git stash apply nome_o_commit_stash_opzionale` : applica uno stash (quello predefinito è l'ultimo, chiamato stash@{0})
- `git stash pop nome_stash_opzionale` : applica uno stash (il predefinito è stash@{0}) e rimuovilo dalla lista degli stash se non ha causato conflitti
- `git stash clear` : rimuovi tutti gli stash

## Links
- More information: https://git-scm.com/docs/git-stash
