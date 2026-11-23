# git cat-file

Fournit des informations sur le contenu ou le type et la taille des objets du dépôt Git.

## Usage
- `git cat-file -s HEAD` : obtient la taille s du commit HEAD en octets
- `git cat-file -t 8c442dc3` : obtient le type t (blob, tree, commit, tag) d'un objet Git spécifié
- `git cat-file -p HEAD~2` : affiche le contenu p d'un objet Git basé sur son type

## Links
- More information: https://git-scm.com/docs/git-cat-file
