# git ls-remote

Commande Git pour répertorier les références dans un dépôt distant en fonction du nom ou de l'URL. Si aucun nom ou URL n'est donné, alors la branche amont configurée sera utilisée, ou l'origine distante si la première n'est pas configurée.

## Usage
- `git ls-remote` : affiche les références du dépôt configuré par défaut
- `git ls-remote --heads` : affiche uniquement les références HEAD du dépôt configuré par défaut
- `git ls-remote -t` : affiche uniquement les tags du dépôt configuré par défaut
- `git ls-remote url-du-dépôt` : affiche les références du dépôt précisé
- `git ls-remote nom-du-dépôt "motif"` : affiche les références du dépôt précisé filtrés par un motif

## Links
- More information: https://git-scm.com/docs/git-ls-remote
