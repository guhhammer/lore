# rename

Renomme un fichier ou un groupe de fichiers à l’aide d’une `regex`. AVERTISSEMENT : cette commande remplacera les fichiers sans demander de confirmation, sauf si l’option de simulation est utilisée. Remarque : cette page concerne la version Perl, également appelée `file-rename`.

## Usage
- `rename 's/depuis/vers/' *.txt` : remplace `depuis` par `vers` dans les noms des fichiers spécifiés
- `rename -n 's/depuis/vers/' *.txt` : effectue une simulation — affiche les modifications qui seraient effectuées sans les appliquer
- `rename 's/\.ancienne$/\.nouvelle/' *.txt` : modifie l’extension
- `rename -f 'y/A-Z/a-z/' *.txt` : convertit les noms en minuscules (utilise `-f` avec les systèmes de fichiers insensibles à la casse)
- `rename -f 's/\b(\w)/\U$1/g' *.txt` : met en majuscule la première lettre de chaque mot du nom
- `rename 's/\s+/_/g' *.txt` : remplace les espaces par des tirets bas

## Links
- More information: https://manned.org/prename
