# mktemp

Crée un fichier ou un répertoire temporaire.

## Usage
- `mktemp` : crée un fichier temporaire vide et affiche son chemin d'accès absolu
- `mktemp -p /chemin/vers/reptemp` : utilise un répertoire personnalisé si `$TMPDIR` n'est pas défini (la valeur par défaut dépend de la plateforme, mais est habituellement `/tmp`)
- `mktemp /tmp/exemple.XXXXXXXX` : utilise un modèle de chemin personnalisé (les `X` sont remplacés par des caractères alphanumériques aléatoires)
- `mktemp -t exemple.XXXXXXXX` : utilise un modèle de nom de fichier personnalisé
- `mktemp -d` : crée un répertoire temporaire vide et affiche son chemin d'accès absolu

## Links
- More information: https://man.openbsd.org/mktemp.1
