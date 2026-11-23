# pacman-upgrade

Met à jour ou ajoute des paquets au système.

## Usage
- `sudo pacman -U chemin/vers/paquet1.pkg.tar.zst chemin/vers/paquet2.pkg.tar.zst ...` : installe un ou des paquets depuis le système de fichiers
- `sudo pacman -U --noconfirm chemin/vers/paquet.pkg.tar.zst` : installe un paquet silencieusement
- `sudo pacman -U --overwrite chemin/vers/fichier chemin/vers/paquet.pkg.tar.zst` : ecris par dessus les fichiers en conflit pendant l'installation du paquet
- `sudo pacman -Ud chemin/vers/paquet.pkg.tar.zst` : installe un paquet sans vérifier les dépendances
- `pacman -Up chemin/vers/paquet.pkg.tar.zst` : affiche ce qui se passerait si la commande était exécutée mais sans agir
- `pacman -Uh` : affiche l'aide

## Links
- More information: https://manned.org/pacman.8
