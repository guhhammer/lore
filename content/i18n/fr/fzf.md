# fzf

Recherche approximative en ligne de commande.

## Usage
- `find chemin/vers/répertoire -type f | fzf` : lance `fzf` sur tous les fichiers du répertoire spécifié
- `ps aux | fzf` : démarre `fzf` pour les processus en cours
- `find chemin/vers/répertoire -type f | fzf -m >chemin/vers/fichier` : sélectionne plusieurs fichiers avec `<Shift Tab>` et écrit dans un fichier
- `fzf -q "query"` : lance `fzf` avec une requête donnée
- `fzf -q "^core go$ | rb$ | py$"` : lance `fzf` sur les entrées qui commencent par `core` et se terminent par `go`, `rb`, ou `py`
- `fzf -q '!pyc travis'` : lance `fzf` sur les entrées qui ne correspondent pas à pyc et qui correspondent exactement à travis

## Links
- More information: https://github.com/junegunn/fzf#usage
