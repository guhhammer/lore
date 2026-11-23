# tr

Convertisseur de caractères : exécute des remplacements basés sur des caractères uniques et des jeux de caractères.

## Usage
- `tr <chemin/vers/fichier caractère_recherché caractère_remplacé` : remplace toutes les occurrences d'un caractère dans un fichier, et affiche le résultat
- `echo texte | tr caractère_recherché caractère_remplacé` : remplace toutes les occurrences d'un caractère dans la sortie d'une autre commande
- `tr <chemin/vers/fichier 'abcd' 'jkmn'` : fais correspondre chaque caractère du premier ensemble au caractère correspondant du second ensemble
- `tr <chemin/vers/fichier -d 'caractères_en_entrée'` : supprime toutes les occurrences de l'ensemble de caractères spécifié dans l'entrée
- `tr <chemin/vers/fichier -s 'caractères_en_entrée'` : comprime une série de caractères identiques en un seul caractère
- `tr <chemin/vers/fichier "[:lower:]" "[:upper:]"` : traduis le contenu d'un fichier en majuscules
- `tr <chemin/vers/fichier -cd "[:print:]"` : supprime les caractères non imprimables d'un fichier

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tr-invocation.html
