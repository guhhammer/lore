# btrfs filesystem

Gérer les systèmes de fichiers btrfs.

## Usage
- `btrfs filesystem usage chemin/vers/point_de_montage_btrfs` : afficher l'utilisation du système de fichiers (affiche les informations détaillées si executé en tant que `root`)
- `sudo btrfs filesystem show chemin/vers/point_de_montage_btrfs` : afficher l'utilisation individuellement pour chaque périphérique
- `sudo btrfs filesystem defragment -v chemin/vers/fichier` : défragmenter un fichier unique sur un système de fichiers btrfs (à éviter lorsqu'un agent de dé-duplication est en cours d'exécution)
- `sudo btrfs filesystem defragment -v -r chemin/vers/dossier` : défragmenter récursivement un dossier (ne franchit pas la limite de sous-volume)
- `sudo btrfs filesystem sync chemin/vers/point_de_montage_btrfs` : force la resynchronisation des blocs de données non écrits sur le ou les disques
- `sudo btrfs filesystem du --summarize chemin/vers/dossier` : afficher un sommaire d'utilisation des disques pour les fichiers dans un dossier, récursivement

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-filesystem.html
