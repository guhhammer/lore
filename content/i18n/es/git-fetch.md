# git fetch

Descarga objetos y referencias de un repositorio remoto.

## Usage
- `git fetch` : descarga los últimos cambios del repositorio remoto upstream por defecto (si se ha establecido)
- `git fetch remote_name` : descarga las ramas nuevas de un repositorio remoto upstream específico
- `git fetch --all` : descarga los últimos cambios de todos los repositorios remotos upstream
- `git fetch -t` : descarga también las etiquetas de un repositorio upstream
- `git fetch -p` : elimina las referencias locales a ramas remotas que han sido eliminadas de upstream
- `git fetch --deepen 2` : ahonda la rama superficial actual en 2 commits

## Links
- More information: https://git-scm.com/docs/git-fetch
