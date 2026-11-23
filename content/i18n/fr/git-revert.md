# git revert

Créer un nouveau commit qui efface les changements du précédent.

## Usage
- `git revert HEAD` : crée un commit qui annule les changements du dernier commit
- `git revert HEAD~4` : crée un commit qui annule les changements des 5 dernier commit
- `git revert master~5..master~2` : crée un commit qui annule les changements de plusieurs commit
- `git revert -n 0c01a9..9a1743` : ne pas créer de nouveau commit, remplacer uniquement dans l'arbre courant

## Links
- More information: https://git-scm.com/docs/git-revert
