# lsd

Liste le contenu d'un répertoire. Commande `ls` de nouvelle génération, écris en Rust.

## Usage
- `lsd -1` : liste les fichiers et les répertoires, un par ligne
- `lsd -a` : liste tous les fichiers et les répertoires, ainsi que ceux cachés, du répertoire courant
- `lsd -F` : liste les fichiers et les répertoires en ajoutant `/` après le nom des répertoires
- `lsd -lha` : liste tous les fichiers et les répertoires dans le format long (permissions, propriétaire, taille dans un format lisible, et date de modification)
- `lsd -lS` : liste les fichiers et les répertoires dans le format long, triés par taille (descendent)
- `lsd -ltr` : liste les fichiers et les répertoires dans le format long, triés par date de modification (plus vieux en premier)
- `lsd -d */` : liste seulement les répertoires
- `lsd --tree -d` : liste récursivement tous les répertoires dans un format arborescent

## Links
- More information: https://github.com/lsd-rs/lsd/blob/main/doc/lsd.md
