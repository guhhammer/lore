# git grep

Cerca stringhe nello storico dei file tracciati nel repository. Supporta molti degli stessi parametri accettati dal comando `grep` tradizionale.

## Usage
- `git grep stringa_ricercata` : cerca una stringa nei file tracciati
- `git grep stringa_ricercata -- file_glob_pattern` : cerca una stringa nei file tracciati che soddisfano un dato pattern
- `git grep --recurse-submodules stringa_ricercata` : cerca una stringa nei file tracciati, sottomoduli inclusi
- `git grep stringa_ricercata HEAD~2` : cerca una stringa in uno dato momento della cronologia del repository
- `git grep stringa_ricercata $(git rev-list --all)` : cerca una stringa in tutti i rami

## Links
- More information: https://git-scm.com/docs/git-grep
