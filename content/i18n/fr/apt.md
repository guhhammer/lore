# apt

Utilitaire de gestion des paquets pour les distributions basées sur Debian. Conçue comme une alternative conviviale à `apt-get` pour une utilisation interactive.

## Usage
- `sudo apt update` : met à jour la liste des paquets et des versions disponibles (il est recommandé de l'exécuter avant les autres commandes `apt`)
- `apt search package` : recherche d'un paquet donné
- `apt list package` : recherche des paquets uniquement par nom (accepte les caractères joker comme `*`)
- `apt show package` : affiche les informations pour un paquet
- `sudo apt install package` : installe un paquet, ou le met à jour avec la dernière version disponible
- `sudo apt remove package` : supprime un paquet (utiliser `purge` à la place supprime également ses fichiers de configuration)
- `sudo apt upgrade` : met à jour tous les paquets installés vers les dernières versions disponibles
- `apt list -i` : liste les paquets installés

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
