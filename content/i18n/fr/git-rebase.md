# git rebase

Rejoue les validations d'une branche par dessus une autre. Communément utilisé pour dupliquer les validations d'une branche dans une autre, en créant de nouvelles validations dans la branche de destination.

## Usage
- `git rebase master` : rejoue les validations de la branche actuelle sur la branche master
- `git rebase -i branche_base_ou_validation` : rejoue les validations interactivement, ce qui permet aux validations d'être re-arrangées, exclues, combinées ou modifiées
- `git rebase --continue` : continue le re-jeu des validations après la résolution d'un conflit
- `git rebase --skip` : continue le re-jeu des validations en sautant la résolution d'un conflit
- `git rebase --abort` : annule l'opération (ex : en cas de conflit)
- `git rebase --onto nouvelle_base ancienne_base` : déplace une partie de la branche actuelle sur une nouvelle base, fournissant l'ancienne base à partir de laquelle commencer
- `git rebase -i HEAD~5` : rejoue les 5 dernières validations, ce qui permet aux validations d'être re-arrangées, exclues, combinées ou modifiées
- `git rebase -X theirs master` : résout automatiquement les conflits en précisant la version à conserver (`theirs` signifie la version des fichiers à privilégier)

## Links
- More information: https://git-scm.com/docs/git-rebase
