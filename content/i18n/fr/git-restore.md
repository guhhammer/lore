# git restore

Restaurez les fichiers de l'arborescence de travail. Nécessite la version 2.23+ de Git.

## Usage
- `git restore chemin/vers/fichier` : restaurer un fichier supprimé à partir du contenu du commit actuel (HEAD)
- `git restore --source commit chemin/vers/fichier` : restaurer un fichier a la version d'un commit spécifié
- `git restore .` : annulez toutes les modifications non validées des fichiers suivis, en revenant au HEAD

## Links
- More information: https://git-scm.com/docs/git-restore
