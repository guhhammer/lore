# git stash

Stocker les modifications Git locales dans une zone temporaire.

## Usage
- `git stash push -m nom_de_stash_optionel` : stocker les changements courants, sauf les fichiers non-suivis
- `git stash -u` : stocker les changements courants, incluant les fichiers non-suivis
- `git stash -p` : stocker les parties d'un fichier interactivement
- `git stash list` : lister tous les stashs (affiche leurs noms, les branches relatives et messages)
- `git stash apply nom_de_stash_ou_de_commit_optionel` : applique un stash (par défaut, le dernier, nommé stash@{0})
- `git stash pop nom_de_stash_optionel` : applique un stash (par défaut le dernier, stash@{0}), et le supprimer de la liste des stashs s'il n'y a pas de conflit
- `git stash clear` : supprime tous les stashs

## Links
- More information: https://git-scm.com/docs/git-stash
