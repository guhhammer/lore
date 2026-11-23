# git rev-list

Liste les révisions (commits) dans l'ordre chronologique inverse.

## Usage
- `git rev-list HEAD` : lister tout les commits dans la branche courante
- `git rev-list --since='2019-12-01 00:00:00' master` : lister tout les commits plus récents qu'une date spécifique, sur une branche spécifique
- `git rev-list --merges commit` : lister tout les commits de merge depuis un commit spécifique

## Links
- More information: https://git-scm.com/docs/git-rev-list
