# git difftool

Affiche les modifications apportées aux fichiers à l'aide d'outils de comparaison externes. Accepte les mêmes options et arguments que Git diff.

## Usage
- `git difftool --tool-help` : liste les outils de comparaison disponibles
- `git config --global diff.tool "meld"` : configure Meld comme outil de comparaison par défaut
- `git difftool --staged` : utilise l'outil de comparaison par défaut pour afficher les fichiers modifiés
- `git difftool -t opendiff validation` : utilise un outil de comparaison spécifique (opendiff) pour afficher les changements depuis une validation

## Links
- More information: https://git-scm.com/docs/git-difftool
