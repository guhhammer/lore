# xargs

Exécute une commande avec des arguments provenant d’un pipe, d’un fichier, etc. L’entrée est traitée comme un bloc de texte unique, puis découpée en éléments séparés par des espaces, tabulations, retours à la ligne et fin de fichier.

## Usage
- `arguments_source | xargs commande` : exécute une commande en utilisant les données d’entrée comme arguments
- `arguments_source | xargs sh -c "commande1 && commande2 | commande3"` : exécute plusieurs commandes chaînées sur les données d’entrée
- `arguments_source | xargs -n 1 commande` : exécute une nouvelle commande pour chaque argument
- `arguments_source | xargs -P 10 -n 1 commande` : augmente le nombre de processus parallèles à 10 (par défaut : 1 ; 0 = autant que possible)
- `arguments_source | xargs -I _ commande _ arguments_optionnels_additionnels` : exécute la commande une fois par ligne d’entrée, en remplaçant le placeholder (ici _) par la ligne
- `arguments_source | xargs -p commande` : demande une confirmation avant exécution (valider avec y ou Y)
- `xargs -a chemin/vers/fichier commande` : lit les arguments depuis un fichier
- `arguments_source | xargs -o commande` : permet à la commande d’accéder au terminal pour une entrée interactive

## Links
- More information: https://www.gnu.org/software/findutils/manual/find.html#xargs-options
