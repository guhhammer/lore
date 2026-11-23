# zoxide

Garde une trace des répertoires les plus utilisés. Utilise un algorithme de classement pour accéder au meilleur résultat.

## Usage
- `zoxide query string` : accède au répertoire avec le meilleur classement qui contient "string" dans son nom
- `zoxide query string1 string2` : accède au répertoire avec le meilleur classement qui contient "string1" puis "string2" dans son nom
- `zoxide query -i` : démarre une recherche de répertoire interactive (nécessite `fzf`)
- `zoxide add chemin/vers/répertoire` : ajoute un répertoire ou incrémente son classement
- `zoxide remove chemin/vers/répertoire` : supprime un répertoire de la base de données de `zoxide`
- `zoxide init bash|elvish|fish|nushell|posix|powershell|tcsh|xonsh|zsh` : génère la configuration du shell pour la mise en place des alias de commandes (`z`, `zi`)

## Links
- More information: https://manned.org/zoxide
