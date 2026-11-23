# git stash

Almacena los cambios locales de Git en un área temporal.

## Usage
- `git stash push -m mensaje_opcional_stash` : almacena los cambios actuales, excepto los archivos nuevos (sin seguimiento)
- `git stash -u` : almacena los cambios actuales, incluyendo los archivos nuevos (sin seguimiento)
- `git stash -p` : selecciona interactivamente partes de los archivos modificados para almacenarlos
- `git stash list` : lista todos los stashes (muestra el nombre del stash, la rama relacionada y el mensaje)
- `git stash show -p stash@{0}` : muestra los cambios como un parche entre el stash (por defecto es `stash@{0}`) y la confirmación de cuando se creó la entrada stash por primera vez
- `git stash apply nombre_opcional_del_stash_o_confirmación` : aplica un stash (por defecto es el último, llamado `stash@{0}`)
- `git stash pop nombre_opcional_stash` : suelta o aplica un stash (por defecto es `stash@{0}`) y lo elimina de la lista de stash si su aplicación no causa conflictos
- `git stash clear` : elimina todos los stashes

## Links
- More information: https://git-scm.com/docs/git-stash
