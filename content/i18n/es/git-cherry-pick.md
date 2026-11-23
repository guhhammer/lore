# git cherry-pick

Aplica los cambios introducidos por confirmaciones existentes a la rama actual. Para aplicar cambios a otra rama, primero utiliza `git checkout` para cambiar a la rama deseada.

## Usage
- `git cherry-pick confirmación` : aplica una confirmación a la rama actual
- `git cherry-pick confirmación_inicial~..confirmación_final` : aplica un rango de confirmaciones de la rama actual (vea también `git rebase --onto`)
- `git cherry-pick confirmación_1 confirmación_2 ...` : aplica múltiples confirmaciones no secuenciales a la rama actual
- `git cherry-pick -n confirmación` : añade los cambios de una confirmación al directorio de trabajo, sin crear una confirmación

## Links
- More information: https://git-scm.com/docs/git-cherry-pick
