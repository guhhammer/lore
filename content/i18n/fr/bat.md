# bat

Affiche et concatène le contenu d'un ou plusieurs fichiers. Un clone de `cat` avec mise en valeur de la syntaxe et intégration avec Git.

## Usage
- `bat fichier` : affiche le contenu d'un fichier sur la sortie standard
- `bat fichier1 fichier2 >fichier_de_destination` : concatène le contenu de plusieurs fichiers vers le fichier de destination
- `bat fichier1 fichier2 >>fichier_de_destination` : ajoute le contenu d'un ficher à la fin du fichier de destination
- `bat -n fichier` : numérote toutes les lignes affichées
- `bat -l json chemin/vers/fichier.json` : affiche le contenu d'un fichier JSON sur la sortie standard avec mise en valeur de la syntaxe
- `bat -L` : affiche tous les langages pris en charge

## Links
- More information: https://manned.org/bat
