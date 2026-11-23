# exa

Une alternative moderne à `ls` (pour lister le contenu de répertoires).

## Usage
- `exa -1` : liste les fichiers, un par ligne
- `exa -a` : liste tous les fichiers, y compris les fichiers cachés
- `exa -l -a` : liste au format long (autorisations, propriété, taille et date de modification) de tous les fichiers
- `exa -r -s size` : liste les fichiers avec le plus volumineux en haut
- `exa -l -T -L 3` : affiche une arborescence de fichiers, sur trois niveaux de profondeur
- `exa -l -s modified` : liste des fichiers triés par date de modification (le plus ancien en premier)
- `exa -l -h --icons --git` : liste les fichiers avec leur en-tête, leur icône et leur statut Git
- `exa --git-ignore` : liste les fichiers sauf ceux mentionnés dans `.gitignore`

## Links
- More information: https://github.com/ogham/exa#command-line-options
