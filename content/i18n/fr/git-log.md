# git log

Affiche un historique de validations.

## Usage
- `git log` : affiche la séquence de validations à partir de l'actuel, dans l'ordre chronologique inverse du dépôt Git dans le répertoire de travail actuel
- `git log -p chemin/vers/fichier_ou_répertoire` : affiche l'historique de fichiers ou répertoires en particulier
- `git log --stat` : affiche la liste des fichiers modifiés pour chaque validation
- `git log --oneline --graph` : affiche un graphique des validations dans la branche actuelle en utilisant uniquement la première ligne de chaque message de validation
- `git log --oneline --decorate --all --graph` : affiche un graphique de tout les validations, étiquettes et branches dans le dépôt entier
- `git log -i --grep chaine_recherché` : affiche uniquement les validations dont le message contient la chaine (insensible à la casse)
- `git log -n number --author "author"` : affiche les N dernières validations d'un utilisateur
- `git log --before "2017-01-29" --after "2017-01-17"` : affiche les validations entre deux dates (aaaa-mm-jj)

## Links
- More information: https://git-scm.com/docs/git-log
