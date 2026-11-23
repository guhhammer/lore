# git send-email

Envoie une collection de correctifs par email. Les correctifs peuvent être spécifiés sous forme de fichiers, de directions ou de liste de révisions.

## Usage
- `git send-email -1` : envoie la dernière validation de la branche actuelle
- `git send-email -1 validation` : envoie une validation spécifique
- `git send-email -10` : envoie de multiples validations de la branche actuelle (ici : 10)
- `git send-email -nombre_validations --compose` : envoie un e-mail de présentation de la série de correctifs
- `git send-email -nombre_validations --annotate` : consulte et modifie l'e-mail de chaque correctif qui va être envoyé

## Links
- More information: https://git-scm.com/docs/git-send-email
