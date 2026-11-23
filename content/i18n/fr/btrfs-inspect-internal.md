# btrfs inspect-internal

Recherche des informations internes concernant un système de fichier btrfs.

## Usage
- `sudo btrfs i dump-s chemin/vers/partition` : affiche les informations du superbloc
- `sudo btrfs i dump-s -a chemin/vers/partition` : affiche les informations sur les superblocs et toutes ses copies
- `sudo btrfs i dump-t chemin/vers/partition` : affiche les meta-informations du système de fichiers
- `sudo btrfs i i n chemin/vers/point_de_montage_btrfs` : affiche la liste des fichiers dans le `n`-ième inode
- `sudo btrfs i lo addresse_logique chemin/vers/point_de_montage_btrfs` : affiche la liste des fichiers à une adresse logique donnée
- `sudo btrfs i t chemin/vers/partition` : affiche les statistiques concernant les arbres de racines, de domaines (extent), de sommes de contrôle (csum) et de système de fichiers

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-inspect-internal.html
