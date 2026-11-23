# git reset

Deshace confirmaciones o desmarca cambios mediante el restablecimiento del actual HEAD de Git al estado especificado. Si se pasa una ruta, funciona como "desmarcar", si se pasa el hash de una confirmación o una rama, funciona como "deshacer" la confirmación.

## Usage
- `git reset` : desmarca todo
- `git reset ruta/al/archivo_o_archivos` : desmarca un archivo o archivos específicos
- `git reset -p ruta/al/archivo` : interactivamente desmarca partes de un archivo
- `git reset HEAD~` : deshace la última confirmación, manteniendo sus cambios, y cualquier otro cambio sin confirmación, en el sistema de archivos
- `git reset --soft HEAD~2` : deshace las últimas dos confirmaciones al añadir sus cambios al índice (p. ej. marcado para confirmación)
- `git reset --hard` : descarta cualquier cambio sin confirmación, marcado o no (se puede `git checkout` solo para los cambios sin marcar)
- `git reset --hard confirmación` : restablece el repositorio a una confirmación específica y descarta a partir de esta los cambios con y sin confirmación, y los marcados

## Links
- More information: https://git-scm.com/docs/git-reset
