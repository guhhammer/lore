# git archive

Crée une archive de fichiers depuis une branche donnée.

## Usage
- `git archive -v HEAD` : crée une archive `.tar` avec le contenu de la HEAD et l'affiche sur la sortie standard
- `git archive -v --format zip HEAD` : crée une archive Zip avec le contenu de la HEAD et l'affiche sur la sortie standard
- `git archive -v -o chemin/vers/fichier.zip HEAD` : pareil que ci-dessus mais écrit dans l'archive spécifiée
- `git archive -o chemin/vers/fichier.tar nom_branche` : crée une archive depuis la dernière validation de la branche spécifiée
- `git archive -o chemin/vers/fichier.tar HEAD:chemin/vers/répertoire` : crée une archive avec le contenu d'un répertoire donné
- `git archive -o chemin/vers/fichier.tar --prefix chemin/vers/cible/ HEAD` : ajoutez un chemin d'accès à chaque fichier pour l'archiver dans un répertoire spécifique

## Links
- More information: https://git-scm.com/docs/git-archive
