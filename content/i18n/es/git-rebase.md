# git rebase

Vuelve a aplicar confirmaciones de una rama en lo más alto de otra rama. Se utiliza comúnmente para "mover" una rama entera a otra base, ya que crea copias de las confirmaciones en una nueva ubicación.

## Usage
- `git rebase nueva_base_rama` : reorganiza la rama actual en lo más alto de otra rama
- `git rebase -i rama_base_objetivo_o_hash_de_la_confirmación` : inicia un rebase interactivo que permite reordenar, omitir, combinar o modificar confirmaciones
- `git rebase --continue` : continúa un rebase que fue interrumpido por una fusión fallida después de editar los archivos con conflictos
- `git rebase --skip` : continúa un rebase que fue pausado para fusionar conflictos saltando la confirmación conflictiva
- `git rebase --abort` : cancela un rebase en proceso (por ej., si es interrumpido por un conflicto de fusión)
- `git rebase --onto base_nueva base_antigua` : mueve parte de la rama actual a una nueva base proporcionando la base antigua para empezar
- `git rebase -i HEAD~5` : reaplica las últimas cinco confirmaciones en su lugar, evita que puedan ser reordenadas, omitidas, combinadas o modificadas
- `git rebase -X theirs nombre_rama` : resuelve automáticamente cualquier conflicto favoreciendo la versión de la rama en la que se está trabajando (en este caso la palabra `theirs` tiene un significado invertido)

## Links
- More information: https://git-scm.com/docs/git-rebase
