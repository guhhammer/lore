# git format-patch

Prépare des fichiers de correctifs, utiles pour les envoyer par courriel.

## Usage
- `git format-patch origin` : crée un fichier de correctif `.patch` nommé automatiquement pour toutes les validations non poussées
- `git format-patch revision_1..revision_2` : crée un fichier correctif `.patch` pour les changements entre 2 révisions
- `git format-patch -3` : crée un fichier correctif `.patch` pour les 3 dernières validations

## Links
- More information: https://git-scm.com/docs/git-format-patch
