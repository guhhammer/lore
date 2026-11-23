# git mergetool

Exécute un utilitaire de différences pour résoudre les conflits de merge.

## Usage
- `git mergetool` : démarre l'outil de différences par défaut
- `git mergetool --tool-help` : liste les outils de différences valides
- `git mergetool -t tool_name` : démarre l'outil de différences en précisant son nom
- `git mergetool -y` : démarre l'outil de différences sans dialogues
- `git mergetool -g` : utilise explicitement l'outil de différences graphique (voir la variable de config `merge.guitool`)
- `git mergetool --no-gui` : utilise explicitement l'outil de différences classique (voir la variable de config `merge.tool`)

## Links
- More information: https://git-scm.com/docs/git-mergetool
