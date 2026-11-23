# git branch

Comando principal de Git para trabajar con ramas.

## Usage
- `git branch -a` : lista todas las ramas (locales y remotas; la rama actual se resalta con `*`)
- `git branch -a --contains hash_de_la_confirmación` : lista las ramas que incluyen una confirmación específica en su historial
- `git branch --show-current` : muestra el nombre de la rama actual
- `git branch nombre_rama` : crea una nueva rama basada en la confirmación actual
- `git branch nombre_de_rama hash_de_la_confirmación` : crea una nueva rama basada en una confirmación específica
- `git branch -m nombre_de_rama_antigua nuevo_nombre_rama` : renombra una rama (para ello no debes tenerla controlada)
- `git branch -d nombre_de_rama` : elimina una rama local (no debes tenerla controlada para hacerlo)
- `git push nombre_remoto -d nombre_de_rama_remota` : elimina una rama remota

## Links
- More information: https://git-scm.com/docs/git-branch
