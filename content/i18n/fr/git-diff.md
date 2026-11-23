# git diff

Affiche les modifications sur les fichiers suivis.

## Usage
- `git diff` : affiche les modifications non indexées
- `git diff HEAD` : affiche toutes les modifications non validées (y compris celles qui sont indexées)
- `git diff --staged` : affiche uniquement les modifications indexées (ajoutées, mais pas encore validées)
- `git diff 'HEAD@{3 months|weeks|days|hours|seconds ago}'` : affiche les modifications de toutes les validations à partir d'une date/heure donnée (expression de dates, ex : "1 week 2 days" pour 1 semaine et 2 jours ou une date ISO)
- `git diff --stat validation` : affiche les statistiques du diff, comme les fichiers modifiés, l'histogramme et le nombre total de lignes ajoutées/supprimées
- `git diff --summary validation` : affiche un résumé des créations de fichiers, renommages ou changements de mode depuis une validation donnée
- `git diff branche_1..branche_2 chemin/vers/fichier` : compare un fichier entre deux branches ou validations
- `git diff autre_branche:chemin/vers/fichier2 chemin/vers/fichier1` : compare différents fichiers de la branche actuelle avec une autre branche

## Links
- More information: https://git-scm.com/docs/git-diff
