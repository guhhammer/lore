# git worktree

Gestiona múltiples árboles de trabajo adjuntos al mismo repositorio.

## Usage
- `git worktree add ruta/al/directorio rama` : crea un nuevo directorio con la rama específicada y se cambia a él
- `git worktree add ruta/al/directorio -b rama_nueva` : crea un nuevo directorio con un nueva rama y se cambia a él
- `git worktree list` : muestra todos los directorios de trabajo adjuntos a este repositorio
- `git worktree prune` : elimina un árbol de trabajo (después de eliminar el directorio del árbol de trabajo)

## Links
- More information: https://git-scm.com/docs/git-worktree
