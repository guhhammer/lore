# eza

Une alternative moderne et régulièrement mise à jour à `ls`, basée sur `exa`.

## Usage
- `eza -1` : liste les fichiers, un par ligne
- `eza -a` : liste tous les fichiers, ainsi que les fichiers cachés
- `eza -al` : liste tous les fichiers avec un format détaillé (permissions, propriétaire, taille et date de modification)
- `eza -r -s size` : liste les fichiers les plus lourds en premier
- `eza -lT -L 3` : affiche une arborescence de fichiers, sur trois niveaux de profondeur
- `eza -l -s modified` : liste les fichiers par date de modification (les plus âgés en premier)
- `eza -lh --icons --git` : liste les fichiers avec leurs en-têtes, icônes et statut Git
- `eza --git-ignore` : ne liste pas les fichiers qui sont inscrits dans `.gitignore`

## Links
- More information: https://github.com/eza-community/eza
