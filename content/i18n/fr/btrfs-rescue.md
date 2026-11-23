# btrfs rescue

Essaie de récupérer un système de fichiers btrfs endommagé.

## Usage
- `sudo btrfs resc ch chemin/vers/partition` : reconstruit les méta-données du système de fichiers (très lent)
- `sudo btrfs resc fix-de chemin/vers/partition` : corrige les problèmes d'alignement de taille de périphérique (e.g. incohérence entre la taille du système de fichiers et le nombre total d'octets empéchant de monter la partition)
- `sudo btrfs resc s chemin/vers/partition` : restaure un superbloc corrompu depuis ses copies correctes (restauration de la racine de l'arbre du système de fichiers)
- `sudo btrfs resc z chemin/vers/partition` : restaure depuis des transactions interrompues (correction des problèmes de re-exécution des messages de journaux)
- `sudo btrfs resc c` : crée un périphérique de contrôle sous `/dev/btrfs-control` quand l'outil `mknod` n'est pas installé

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-rescue.html
