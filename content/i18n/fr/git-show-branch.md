# git show-branch

Affiche les branches et leurs validations.

## Usage
- `git show-branch nom_branche|ref|validation` : affiche un résumé de la dernière validation dans la branche
- `git show-branch nom_branche1|ref1|validation1 nom_branche2|ref2|validation2 ...` : compare des validations avec plusieurs validations ou branches
- `git show-branch -r` : compare toutes les branches de suivi distantes
- `git show-branch -a` : compare les branches locales et les branches de suivi distantes
- `git show-branch -a --list` : liste les dernières validations sur toutes les branches
- `git show-branch --current validation|nom_branche|ref` : compare une branche spécifique à la branche actuelle
- `git show-branch --sha1-name --current actuel|nom_branche|ref` : affiche le nom de la validation au lieu du nom relatif
- `git show-branch --more 5 nom_branche1|ref1|validation1 nom_branche2|ref2|validation2 ...` : continue l'affichage d'un certain nombre de validations au-delà de l'ancêtre commun

## Links
- More information: https://git-scm.com/docs/git-show-branch
