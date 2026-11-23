# pacman-deptest

Vérifie la satisfaction des dépendances et renvoie celles qui ne le sont pas.

## Usage
- `pacman -T paquet1 paquet2 ...` : affiche les noms des paquets qui ne sont pas installés dans la liste
- `pacman -T "bash>=5"` : vérifie que le paquet installé a une version supérieure ou égale
- `pacman -T "bash>5"` : vérifie qu'une version ultérieure du paquet est installée
- `pacman -Th` : affiche l'aide

## Links
- More information: https://manned.org/pacman.8
