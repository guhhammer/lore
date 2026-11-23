# btrfs

Système de fichiers basé sur le principe de copie à l’écriture ("copy-on-write", souvent désigné par son sigle anglais COW) pour Linux. Certaines sous-commandes comme `device` ont leur propre documentation.

## Usage
- `sudo btrfs su c chemin/vers/sous_volume` : crée un sous-volume
- `sudo btrfs su l chemin/vers/point_de_montage` : liste les sous-volumes
- `sudo btrfs f df chemin/vers/point_de_montage` : affiche les informations d'utilisation d'espace
- `sudo btrfs qu e chemin/vers/sous_volume` : active les quotas
- `sudo btrfs qg s chemin/vers/sous_volume` : affiche les quotas

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs.html
