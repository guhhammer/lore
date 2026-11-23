# pacman

Outil de gestion de paquets sur Arch Linux.

## Usage
- `sudo pacman -Syu` : synchronise et mets à jour tous les paquets
- `sudo pacman -S nom_paquet` : installe un nouveau paquet
- `sudo pacman -Rs nom_paquet` : efface un paquet et ses dépendances
- `pacman -Ss "motif_recherché"` : recherche dans la base de données des paquets à l’aide d’une `regex` ou d’un mot-clé
- `pacman -F "nom_fichier"` : recherche dans la base de données des paquets contenant un certain fichier
- `pacman -Qe` : liste seulement les paquets installés explicitement et leurs versions
- `pacman -Qtdq` : liste les paquets orphelins (installés comme dépendances, mais qui ne sont requis par aucun paquet)
- `sudo pacman -Scc` : vide le cache des paquets pour libérer de l'espace

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
