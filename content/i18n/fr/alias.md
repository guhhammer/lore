# alias

Créé des alias -- mots qui sont remplacés par une commande textuelle. Les alias expirent avec la session courante shell, sauf s'il a été défini dans le fichier de configuration shell, par exemple `~/.bashrc`.

## Usage
- `alias` : liste tous les alias
- `alias mot="commande"` : crée un alias générique
- `alias mot` : affiche la commande associée à un alias donné
- `unalias mot` : enlève un alias
- `alias rm="rm -i"` : transforme `rm` en une commande intéractive
- `alias la="ls -a"` : crée `la` comme un raccourci de `ls -a`

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias
