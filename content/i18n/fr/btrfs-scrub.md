# btrfs scrub

Examine un système de fichiers btrfs pour vérifier l'intégrité de ses données. Il est recommandé de faire tourner une vérification tous les mois.

## Usage
- `sudo btrfs sc start chemin/vers/point_montage_btrfs` : démarre un examen
- `sudo btrfs sc status chemin/vers/point_montage_btrfs` : affiche le statut d'un examen en cours, ou du dernier examen complété
- `sudo btrfs sc c chemin/vers/point_montage_btrfs` : arrête un examen en cours
- `sudo btrfs sc r chemin/vers/point_montage_btrfs` : reprend un examen précédemment stoppé
- `sudo btrfs sc start -B chemin/vers/point_montage_btrfs` : démarre un examen, mais attend qu'il termine avant de rendre la main
- `sudo btrfs sc start -q chemin/vers/le/point_montage_btrfs` : démarre un examen en mode silencieux (n'affiche ni erreurs ni statistiques)

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-scrub.html
