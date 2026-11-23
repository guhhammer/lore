# git shortlog

Récapitule la sortie de `git log`.

## Usage
- `git shortlog` : affiche un résumé de tous les commits effectués, regroupés par ordre alphabétique par nom d'auteur
- `git shortlog -n` : affiche un résumé de tous les commits effectués, regroupés par le nombre de commits effectués
- `git shortlog -c` : affiche un résumé de tous les commits effectués, regroupés par le nom et l'email de l'utilisateur
- `git shortlog HEAD~5..HEAD` : affiche un résumé des 5 derniers commits effectués
- `git shortlog -sne` : affiche tous les utilisateurs, emails et le nombre de commits dans la branche
- `git shortlog -sne --all` : affiche tout les utilisateurs, emails et le nombre de commits dans toutes les branches

## Links
- More information: https://git-scm.com/docs/git-shortlog
