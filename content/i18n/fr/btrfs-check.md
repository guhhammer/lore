# btrfs check

Vérifie l'état, ou réparer un système de fichiers de type btrfs.

## Usage
- `sudo btrfs c chemin/vers/partition` : vérifie l'état d'un système de fichiers btrfs
- `sudo btrfs c --repair chemin/vers/partition` : vérifie l'état et réparer d'un système de fichiers btrfs (dangereux)
- `sudo btrfs c -p chemin/vers/partition` : affiche la progression de vérification en cours
- `sudo btrfs c --check-data-csum chemin/vers/partition` : vérifie la somme de contrôle de chaque bloc de données (si le système de fichiers à été correctement vérifié)
- `sudo btrfs c -s n chemin/vers/partition` : utilise le `n`-ième super-bloc (`n` peut-être `0`, `1` ou `2`)
- `sudo btrfs c --repair --init-csum-tree chemin/vers/partition` : reconstruit l'arbre des sommes de contrôle (checksum tree)
- `sudo btrfs c --repair --init-extent-tree chemin/vers/partition` : reconstruit l'arbre des domaines (extent tree)

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-check.html
