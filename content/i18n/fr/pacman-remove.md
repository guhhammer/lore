# pacman-remove

Supprime des paquets.

## Usage
- `sudo pacman -Rs paquet` : supprime un paquet et ses dépendances
- `sudo pacman -Rsn paquet` : supprime un paquet, ses dépendances et ses fichiers de configuration
- `sudo pacman -R --noconfirm paquet` : supprime un paquet silencieusement
- `sudo pacman -Rsn $(pacman -Qdtq)` : supprime les paquets orphelins (installés en tant que dépendance mais requis par aucun paquet installé)
- `sudo pacman -Rc paquet` : supprime un paquet et les paquets qui en dépendent
- `pacman -Rp paquet` : affiche les paquets qui seraient affectés par la commande sans agir
- `pacman -Rh` : affiche l'aide

## Links
- More information: https://manned.org/pacman.8
