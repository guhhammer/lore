# git push

Pousse les validations vers un dépôt distant.

## Usage
- `git push` : envoie les modifications locales dans la branche actuelle vers sa contrepartie distante
- `git push nom_distant branche_locale` : envoie les modifications locales d'une branche spécifique vers sa contrepartie distante
- `git push -u nom_distant branche_locale` : envoie les modifications d'une branche locale spécifique vers sa contrepartie distante et définit cette dernière comme cible par défaut pour les opérations push/pull de la branche locale
- `git push nom_distant branche_locale:branche_distante` : envoie les modifications d'une branche locale spécifique vers une branche distante spécifique
- `git push --all nom_distant` : envoie les modifications locales sur toutes les branches locales vers leur contrepartie sur le dépôt distant
- `git push nom_distant -d branche_distante` : supprime une branche dans un dépôt distant
- `git push --prune nom_distant` : supprime les branches distantes qui n'ont pas de contrepartie locale
- `git push --tags` : publie les étiquettes qui ne sont pas encore sur le dépôt distant

## Links
- More information: https://git-scm.com/docs/git-push
