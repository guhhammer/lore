# btrfs restore

Tente de récupérer des fichiers depuis un système de fichiers btrfs endommagé.

## Usage
- `sudo btrfs rest chemin/vers/peripherique_btrfs chemin/vers/repertoire` : restaure tous les fichiers depuis un système de fichier btrfs vers un répertoire cible indiqué
- `sudo btrfs rest -D chemin/vers/peripherique_btrfs chemin/vers/repertoire` : liste (sans écriture) les fichiers qui peuvent être récupérés depuis un système de fichiers btrfs
- `sudo btrfs rest --path-regex regex -c chemin/vers/peripherique_btrfs chemin/vers/repertoire` : restaure les fichiers correspondants à une `regex` donnée (non sensible à la casse) à restaurer depuis un système de fichiers btrfs (tous les répertoires parents des fichiers doivent correspondre également à la `regex`)
- `sudo btrfs rest -t bytenr chemin/vers/peripherique_btrfs chemin/vers/repertoire` : restaure les fichiers depuis un système de fichiers btrfs en utilisant un arbre racine spécifique `bytenr` (voir `btrfs-find-root`)
- `sudo btrfs rest -m -x -S -o chemin/vers/peripherique_btrfs chemin/vers/repertoire` : restaure les fichiers depuis un système de fichiers btrfs (avec métadonnées, attributs étendus, et liens symboliques) en écrivant par dessus les fichiers déjà existants dans le répertoire cible

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-restore.html
