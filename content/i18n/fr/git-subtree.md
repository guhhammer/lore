# git subtree

Fusionne des sous-arbres ou divise un dépôt en sous-arbres.

## Usage
- `git subtree add -P chemin/vers/répertoire --squash url_dépôt nom_branche` : ajoute un dépôt Git en tant que sous-arbre et fusionne les validations
- `git subtree pull -P chemin/vers/répertoire url_dépôt nom_branche` : met à jour le sous-arbre avec sa dernière validation
- `git subtree merge -P chemin/vers/répertoire --squash url_dépôt nom_branche` : fusionne les modifications récentes jusqu'à la dernère validation du sous-arbre dans le sous-arbre
- `git subtree push -P chemin/vers/répertoire url_dépôt nom_branche` : pousse les commits vers le dépôt d'un sous-arbre
- `git subtree split -P chemin/vers/répertoire url_dépôt -b nom_branche` : extrait un nouvel historique de projet de l'historique d'un sous-arbre

## Links
- More information: https://manned.org/git-subtree
