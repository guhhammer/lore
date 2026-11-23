# git clean

Elimina archivos sin rastrear del árbol de trabajo.

## Usage
- `git clean` : elimina archivos que no son rastreados por Git
- `git clean -i` : elimina interactivamente archivos que no son rastreados por Git
- `git clean -n` : muestra que archivos serían borrados sin llegar a borrarlos
- `git clean -f` : elimina forzosamente los archivos que no son rastreados por Git
- `git clean -f -d` : elimina forzosamente los directorios que no son rastreados por Git
- `git clean -x` : elimina archivos sin rastrear, incluyendo los archivos ignorados en `.gitignore` y los excluidos en `.git/info/exclude`

## Links
- More information: https://git-scm.com/docs/git-clean
