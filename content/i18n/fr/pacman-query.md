# pacman-query

Fait des requêtes dans la base de données des paquets installés.

## Usage
- `pacman -Q` : liste les paquets installés et leur version
- `pacman -Qeq` : liste uniquement les paquets installés explicitement
- `pacman -Qo fichier` : trouve le paquet propriétaire d'un fichier
- `pacman -Qi paquet` : affiche des informations sur un paquet installé
- `pacman -Ql paquet` : liste les fichiers appartenant à un paquet
- `pacman -Qdtq` : liste les paquets orphelins (installés en tant que dépendances mais requis par aucun paquet installé)
- `pacman -Qm` : liste les paquets installés ne se trouvant pas dans les dépôts
- `pacman -Qu` : liste les paquets périmés

## Links
- More information: https://manned.org/pacman.8
