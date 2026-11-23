# git push

Envía (push) los commits al repositorio remoto.

## Usage
- `git push` : envía los cambios locales en la rama actual a la misma rama en el remoto
- `git push nombre_remoto rama_local` : envía los cambios locales de una rama específica a la misma rama en el remoto
- `git push -u nombre_remoto rama_remota` : publica la rama actual en el repositorio remoto y establece el nombre remoto de la rama
- `git push nombre_remoto rama_local:rama_remota` : envía los cambios locales de una rama específica a una rama específica en el remoto
- `git push --all nombre_remoto` : envía los cambios de todas las ramas locales a sus respectivas ramas en el repositorio remoto
- `git push nombre_remoto -d rama_remota` : elimina una rama en el repositorio remoto
- `git push --prune nombre_remoto` : elimina las ramas remotas que no están en el repositorio local
- `git push --tags` : publica las etiquetas que aún no están en el repositorio remoto

## Links
- More information: https://git-scm.com/docs/git-push
