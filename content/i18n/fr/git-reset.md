# git reset

Enlève des validations ou des modifications en réinitialisant la tête Git à l'état spécifié. Si un chemin est passé en paramètre, Git reset fonctionne comme «unstage»; si un hash de validation ou une branche est passé en paramètre, Git reset fonctionne comme «uncommit».

## Usage
- `git reset` : désindexe tout
- `git reset chemin/vers/fichier1 chemin/vers/fichier2 ...` : désindexe des fichiers spécifiques
- `git reset -p chemin/vers/fichier` : désindexe certaines parties d'un fichier en mode interactif
- `git reset HEAD~` : annule la dernière validation, mais garde les modifications effectuées (ainsi que toute autre modification non validée) dans le système de fichiers
- `git reset --soft HEAD~2` : défait les deux dernières validations, et ajoute leurs modifications à l'index (indexées pour validation)
- `git reset --hard` : enlève toutes les modifications qui n'ont pas été validées, qu'elles soient indexées ou non (pour enlever uniquement les modifications désindexées, utiliser `git checkout`)
- `git reset --hard validation` : réinitialise le dépôt à une validation spécifique en retirant les modifications validées, indexées, et désindexées depuis cette validation

## Links
- More information: https://git-scm.com/docs/git-reset
