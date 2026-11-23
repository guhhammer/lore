# git branch

Commande Git principale pour travailler avec des branches.

## Usage
- `git branch -a` : liste toutes les branches (locale et distantes)
- `git branch --show-current` : affiche le nom de la branche courante
- `git branch nom_de_branche` : crée une nouvelle branche depuis le commit courant
- `git branch nom_de_branche commit_hash` : crée une nouvelle branche depuis un commit en particulier
- `git branch -m ancien_nom_de_branche nouveau_nom_de_branche` : renommer une branche (ne pas se trouver sur la branche pour le faire)
- `git branch -d nom_de_branche` : supprimer un branche locale (ne pas se trouver sur la branche pour le faire)
- `git push nom_distant -d nom_de_branche_distante` : supprimer une branche distante

## Links
- More information: https://git-scm.com/docs/git-branch
