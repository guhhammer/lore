# git mktree

Construye un objeto árbol usando texto formateado `ls-tree`.

## Usage
- `git mktree` : construye un objeto árbol y verifica que el hash de cada entrada del árbol identifica un objeto existente
- `git mktree --missing` : permite que falten objetos
- `git mktree -z` : lee la salida terminada en NUL (carácter cero) del objeto árbol (`ls-tree -z`)
- `git mktree --batch` : permite la creación de múltiples objetos árbol
- `git <ruta/a/árbol.txt mktree` : ordena y construye un árbol a partir de `stdin` (se requiere un formato de salida de `git ls-tree` no recursivo)

## Links
- More information: https://git-scm.com/docs/git-mktree
