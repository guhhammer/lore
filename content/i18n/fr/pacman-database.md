# pacman-database

Interagit avec les bases de données des paquets Arch Linux. Modifie certains attributs des paquets installés.

## Usage
- `sudo pacman -D --asdeps paquet` : marque un paquet comme étant installé en tant que dépendance
- `sudo pacman -D --asexplicit paquet` : marque un paquet comme étant explicitement installé
- `pacman -Dk` : vérifie les dépendances de tous les paquets installés
- `pacman -Dkk` : vérifie que toutes les dépendances des paquets installés sont disponibles dans les dépôts
- `pacman -Dkq` : N'affiche que les messages d'erreur
- `pacman -Dh` : affiche l'aide

## Links
- More information: https://manned.org/pacman.8
