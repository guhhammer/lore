# git ls-tree

Muestra los contenidos de un objeto árbol.

## Usage
- `git ls-tree nombre_de_la_rama` : lista el contenido del árbol en una rama
- `git ls-tree -r hash_de_la_confirmación` : lista el contenido del árbol en una confirmación (recursivo en subárboles)
- `git ls-tree --name-only hash_de_la_confirmación` : lista solo los nombres de archivos del árbol en una confirmación
- `git ls-tree -r --name-only HEAD | tree --fromfile` : lista el nombre de los archivos en la rama actual en forma de árbol (Nota: la opción `--fromfile` no está disponible en el comando `tree` de Windows)

## Links
- More information: https://git-scm.com/docs/git-ls-tree
