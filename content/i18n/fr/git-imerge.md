# git imerge

Génère un `git merge` ou un `git rebase` entre deux branches de manière incrémentale. Les conflits entre les branches sont suivis en paires de validations individuelles, pour simplifier la résolution des conflits.

## Usage
- `git imerge rebase branche_sur_laquelle_rebaser` : démarre un imerge rebase (se place dans la branche à rebaser d'abord)
- `git imerge merge branche_à_fusionner` : démarre imerge merge (se place dans la branche depuis laquelle fusionner d'abord)
- `git imerge diagram` : affiche le diagramme ASCII du merge ou rebase en cours
- `git imerge continue --no-edit` : continue l'opération après une résolution de conflit (d'abord `git add` les fichiers en conflits)
- `git imerge finish` : termine l'opération imerge après la résolution de tous les conflits
- `git imerge remove && git checkout branche_précédente` : annule l'opération et retourne à la branche précédente

## Links
- More information: https://github.com/mhagger/git-imerge
