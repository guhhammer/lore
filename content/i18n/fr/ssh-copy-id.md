# ssh-copy-id

Dépose une clé publique sur une machine distante, dans les clés autorisées `authorized_keys`.

## Usage
- `ssh-copy-id nom_utilisateur@hote_distant` : dépose toutes les clés publiques sur la machine distante
- `ssh-copy-id -i chemin/vers/certificat nom_utilisateur@hote_distant` : dépose une clé publique spécifique sur la machine distante
- `ssh-copy-id -i chemin/vers/certificat -p port nom_utilisateur@hote_distant` : dépose une clé publique spécifique sur la machine distante en utilisant un port particulier

## Links
- More information: https://manned.org/ssh-copy-id
