# chgrp

Change la propriété de groupe des fichiers et des répertoires.

## Usage
- `chgrp groupe chemin/vers/fichier_ou_répertoire` : change le groupe propriétaire d'un fichier/répertoire
- `chgrp -R groupe chemin/vers/répertoire` : change récursivement le groupe propriétaire d'un répertoire et de son contenu
- `chgrp -h groupe chemin/vers/lien_symbolique` : change le groupe propriétaire d'un lien symbolique
- `chgrp --reference chemin/vers/fichier_référence chemin/vers/fichier_ou_répertoire` : modifie le groupe propriétaire d'un fichier/répertoire pour qu'il corresponde à un fichier de référence
