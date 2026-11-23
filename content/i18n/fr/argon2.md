# argon2

Calcule les hachés cryptographiques d'Argon2.

## Usage
- `echo "mot_de_passe" | argon2 "sel"` : calcule un haché avec un mot de passe et un sel avec les paramètres par défaut
- `echo "mot_de_passe" | argon2 "sel" -d|i|id` : calcule un haché avec l'algorithme spécifié
- `echo "mot_de_passe" | argon2 "sel" -e` : affiche le haché de sortie sans informations supplémentaires
- `echo "mot_de_passe" | argon2 "sel" -t 5 -m 20 -p 7` : calcule un haché avec des temps d'itération, l'utilisation de la mémoire et des paramètres de parallélisme donnés

## Links
- More information: https://github.com/P-H-C/phc-winner-argon2#command-line-utility
