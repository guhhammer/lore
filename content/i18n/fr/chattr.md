# chattr

Modifie les attributs des fichiers ou des répertoires.

## Usage
- `sudo chattr +i chemin/vers/fichier_ou_répertoire` : rend un fichier ou un répertoire immuable aux modifications et à la suppression, même par le superutilisateur
- `sudo chattr -i chemin/vers/fichier_ou_répertoire` : rend un fichier ou un répertoire muable
- `sudo chattr -R +i chemin/vers/répertoire` : rend Récursivement un répertoire entier et son contenu immuables
- `chattr +F chemin/vers/répertoire` : indique qu'un répertoire et ses fichiers doivent être interprétés de manière insensible à la casse
- `sudo chattr +a chemin/vers/fichier` : configure un fichier pour qu'il n'autorise que l'ajout de données

## Links
- More information: https://manned.org/chattr
