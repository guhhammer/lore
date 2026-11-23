# tail

Afficher la fin d'un fichier.

## Usage
- `tail chemin/vers/fichier` : affiche les 10 dernières lignes d'un fichier
- `tail chemin/vers/fichier1 chemin/vers/fichier2 ...` : affiche les 10 dernières lignes de plusieurs fichiers
- `tail -5 chemin/vers/fichier` : affiche les 5 dernières lignes d'un fichier
- `tail -n +ligne chemin/vers/fichier` : affiche tout le contenu d'un fichier à partir d'une `ligne` spécifique
- `tail -c nombre chemin/vers/fichier` : affiche un `nombre` spécifique d'octets à partir de la fin d'un fichier
- `tail -f chemin/vers/fichier` : affiche les dernières lignes d'un fichier et continuer à le lire jusqu'à ce que l'on appuie sur `<Ctrl c>`
- `tail -F chemin/vers/fichier` : continue à lire le fichier jusqu'à ce que l'on appuie sur `<Ctrl c>`, même si le fichier est inaccessible
- `tail -n nombre -s secondes -f chemin/vers/fichier` : affiche les `nombre` dernières lignes d'un fichier et actualiser toutes les `secondes` secondes

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tail-invocation.html
