# pacman-key

Script enrobeur pour GnuPG utilisé pour gérer le trousseau de clés de pacman.

## Usage
- `sudo pacman-key --init` : initialise le trousseau de clés de pacman
- `sudo pacman-key --populate` : ajoute les clés par défaut pour Arch Linux
- `pacman-key -l` : liste les clés publiques du trousseau de clés
- `sudo pacman-key -a chemin/vers/fichier` : ajoute les clés contenues dans le fichier spécifié
- `sudo pacman-key -r "uid|nom|email"` : reçois une clé depuis un serveur de clés
- `pacman-key -f "uid|nom|email"` : affiche l'empreinte d'une clé du trousseau de clés
- `sudo pacman-key --lsign-key "uid|nom|email"` : signe, localement, une clé du trousseau de clés
- `sudo pacman-key -d "uid|nom|email"` : supprime une clé

## Links
- More information: https://manned.org/pacman-key
