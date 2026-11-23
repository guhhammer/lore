# ssh-keyscan

Récupère les clés SSH publiques de machines distantes.

## Usage
- `ssh-keyscan hote_distant` : récupère toutes les clés d'une machine distante
- `ssh-keyscan -p port hote_distant` : récupère toutes les clés d'une machine distante en écoutant sur un port particulier
- `ssh-keyscan -t rsa,dsa,ecdsa,ed25519 hote_distant` : récupère un type particulier de clés d'une machine distante
- `ssh-keyscan -H hote_distant >>~/.ssh/known_hosts` : met à jour manuellement le fichier `known_hosts` des hôtes connus avec l'empreinte d'une

## Links
- More information: https://man.openbsd.org/ssh-keyscan
