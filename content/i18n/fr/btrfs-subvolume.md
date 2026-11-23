# btrfs subvolume

Gestion des sous-volumes et instantanés btrfs.

## Usage
- `sudo btrfs su c chemin/vers/nouveau_sous_volume` : crée un nouveau sous-volume vide
- `sudo btrfs su l chemin/vers/systeme_de_fichiers_btrfs` : liste tous les sous-volumes et instantanés du système de fichiers indiqué
- `sudo btrfs su d chemin/vers/sous_volume` : supprime un sous-volume
- `sudo btrfs su sn -r chemin/vers/sous_volume_source chemin/vers/sous_volume_cible` : crée un instantané en lecture seule d'un sous-volume existant
- `sudo btrfs su sn chemin/vers/sous_volume_source chemin/vers/sous_volume_cible` : crée un instantané en lecture et écriture d'un sous-volume existant
- `sudo btrfs su sh chemin/vers/sous_volume` : affiche les informations détaillées d'un sous-volume

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-subvolume.html
