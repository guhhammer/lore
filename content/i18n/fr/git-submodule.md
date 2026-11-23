# git submodule

Inspecter, mettre à jour et manager des sous-modules.

## Usage
- `git submodule update --init --recursive` : installer un sous-module depuis le dépôt courant
- `git submodule add repository_url` : ajout d'un dépôt Git en tant que sous-module
- `git submodule add repository_url chemin/vers/repertoire` : ajout d'un dépôt Git en tant que sous-module à un répertoire donné
- `git submodule foreach git pull` : mettre à jour tout les sous-modules à leur dernier commit

## Links
- More information: https://git-scm.com/docs/git-submodule
