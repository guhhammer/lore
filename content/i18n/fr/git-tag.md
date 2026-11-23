# git tag

Créer, lister, vérifier et supprimer des tags. Un tag est une référence statique vers un commit.

## Usage
- `git tag` : lister tout les tags
- `git tag nom_d_etiquette` : créer un tag avec le nom donné pointant vers le commit actuel
- `git tag nom_d_etiquette commit` : créer un tag avec le nom donné pointant vers un commit spécifié
- `git tag nom_d_etiquette -m message_d_etiquette` : créer un tag annoté avec le message spécifié
- `git tag -d nom_d_etiquette` : supprimer le tag avec le nom spécifié
- `git fetch -t` : mettre à jour les tags depuis l'origine
- `git tag --contains commit` : liste toutes les tags dont les ancêtres incluent un commit donné

## Links
- More information: https://git-scm.com/docs/git-tag
