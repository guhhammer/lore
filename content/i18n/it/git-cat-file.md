# git cat-file

Visualizza il contenuto di un oggetto Git nel repository o mostrane dimensione e tipo.

## Usage
- `git cat-file -s HEAD` : mostra la dimensione del commit HEAD in byte
- `git cat-file -t 8c442dc3` : mostra il tipo (blob, albero, commit, tag) di un oggetto Git
- `git cat-file -p HEAD~2` : stampa il contenuto di un oggetto Git, formattato in base al tipo

## Links
- More information: https://git-scm.com/docs/git-cat-file
