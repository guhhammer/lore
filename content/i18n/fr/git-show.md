# git show

Affiche différents types d'objets Git (validations, étiquettes, etc.).

## Usage
- `git show` : affiche des informations sur la dernière validation (haché, message, modifications et autres métadonnées)
- `git show validation|étiquette|branche` : affiche les informations sur une validation, étiquette, ou branche spécifique (comme `HEAD` pour la validation la plus récente)
- `git show --summary validation` : affiche uniquement la liste des fichiers ajoutés, renommés, ou supprimés
- `git show -w` : ignore les espaces lors de la comparaison de lignes
- `git show --oneline -s validation` : affiche le message d'une validation sur une seule ligne, en supprimant la sortie diff
- `git show --stat validation` : affiche les statistiques du diff pour les fichiers modifiés (comme des lignes ajoutées ou supprimées)
- `git show --name-only validation` : affiche une liste simplifiée de tous les fichiers modifiés dans une validation (modifiés, ajoutés, et supprimés)
- `git show révision:chemin/vers/fichier` : affiche le contenu d'un fichier tel qu'il était à une révision donnée (par exemple, branche, étiquette ou validation)

## Links
- More information: https://git-scm.com/docs/git-show
