# babel

Un transpilateur qui convertit du code JavaScript avec la syntaxe ES6/ES7 en syntaxe ES5.

## Usage
- `babel chemin/vers/fichier` : transpile une fichier spécifié et affiche le résultat dans la sortie standard
- `babel chemin/vers/fichier_entrée --out-file chemin/vers/fichier_sortie` : transpile un fichier spécifié et enregistre le résultat dans un autre fichier
- `babel chemin/vers/fichier_entrée --watch` : transpile un fichier d'entrée à chaque fois qu'il est modifié
- `babel chemin/vers/répertoire` : transpile tout un répertoire et ses fichiers
- `babel chemin/vers/répertoire --ignore fichiers_ignorés` : ignore des fichiers (séparés par une virgule) dans un répertoire
- `babel chemin/vers/fichier_entrée --minified` : transpile et minifie la sortie
- `babel chemin/vers/fichier_entrée --presets pré-configurations` : sélectionne un lot de pré-configuration pour le formatage de sortie
- `babel --help` : affiche l'aide

## Links
- More information: https://babeljs.io/docs/babel-cli
