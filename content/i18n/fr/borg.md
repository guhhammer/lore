# borg

Outil de sauvegarde avec dé-duplication. Crée des sauvegardes distantes ou locales qui peuvent être montées comme un système de fichiers.

## Usage
- `borg init -e repokey|keyfile|none|... /chemin/vers/repertoire_du_depot` : initialise un dépôt local
- `borg create --progress /chemin/vers/repertoire_du_depot::Lundi /chemin/vers/repertoire_source` : sauvegarde un répertoire dans le dépôt en créant une archive appelée "Lundi"
- `borg list /chemin/vers/repertoire_du_depot` : liste toutes les archives d'un dépôt
- `borg extract utilisateur@hote:/chemin/vers/repertoire_du_depot::Lundi chemin/vers/repertoire_destination --exclude '*.ext'` : extrait un répertoire donné de l'archive nommée "Lundi" à partir d'un dépôt distant tout en excluant tous les fichiers *.ext
- `borg prune --keep-within 7d --list /chemin/vers/repertoire_du_depot` : nettoie un dépôt en effaçant toutes les archives âgées de plus de 7 jours tout en affichant les changements
- `borg mount /chemin/vers/repertoire_du_depot::Lundi /chemin/vers/point_de_montage` : monte un dépôt comme un système de fichiers FUSE
- `borg create --help` : affiche l'aide sur la création d'archives

## Links
- More information: https://borgbackup.readthedocs.io/en/stable/usage/general.html
