# git cherry-pick

Appliquer les modifications introduites par les commits existants à la branche actuelle. Pour appliquer les changements a une autre branche, utiliser d'abord `git checkout` pour basculer sur la branche désirée.

## Usage
- `git cherry-pick commit` : applique un commit à la branche courante
- `git cherry-pick start_commit~..end_commit` : appliquer une plage de commits à la branche courante (voir aussi `git rebase --onto`)
- `git cherry-pick commit_1 commit_2 ...` : appliquer plusieurs commits non séquentiels à la branche courante
- `git cherry-pick -n commit` : appliquer les changements d'un commit à la branche courante sans créer de commit

## Links
- More information: https://git-scm.com/docs/git-cherry-pick
