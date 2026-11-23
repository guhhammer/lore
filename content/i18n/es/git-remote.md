# git remote

Gestiona el conjunto de repositorios rastreados ("remotos").

## Usage
- `git remote -v` : muestra una lista de los remotos existentes, sus nombres y URL
- `git remote show nombre_remoto` : muestra información de un remoto
- `git remote add nombre_remoto url_remoto` : añade un remoto
- `git remote set-url nombre_remoto nueva_url` : cambia la URL de un remoto (utiliza `--add` para mantener la URL existente)
- `git remote get-url nombre_del_remoto` : muestra la URL de un remoto
- `git remote remove nombre_remoto` : elimina un remoto
- `git remote rename nombre_antiguo nombre_nuevo` : renombra un remoto

## Links
- More information: https://git-scm.com/docs/git-remote
