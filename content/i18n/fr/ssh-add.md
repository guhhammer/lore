# ssh-add

Gère les clés SSH enregistrées dans l'agent SSH `ssh-agent`. S'assurer que `ssh-agent` est en fonctionnement pour enregistrer des clés.

## Usage
- `ssh-add` : ajoute les clés présentes dans `~/.ssh` (clés par défaut) à l'agent SSH
- `ssh-add chemin/vers/clé_privée` : ajoute une clé spécifique à l'agent SSH
- `ssh-add -l` : liste les empreintes des clés actuellement enregistrées
- `ssh-add -d chemin/vers/clé_privée` : supprime une clé de l'agent SSH
- `ssh-add -D` : supprime toutes les clés enregistrées de l'agent SSH
- `ssh-add -K chemin/vers/clé_privée` : ajoute une clé spécifique à l'agent SSH et au trousseau de clés

## Links
- More information: https://man.openbsd.org/ssh-add
