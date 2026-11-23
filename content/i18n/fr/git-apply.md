# git apply

Applique un correctif à un fichier et/ou à l index.

## Usage
- `git apply -v chemin/vers/fichier` : affiche les messages à propos des fichiers corrigés
- `git apply --index chemin/vers/fichier` : applique le correctif et ajoute les fichiers à l index
- `curl -L https://example.com/file.patch | git apply` : applique un correctif depuis une source distante
- `git apply --stat --apply chemin/vers/fichier` : affiche les différences résultantes et applique le correctif
- `git apply -R chemin/vers/fichier` : applique le correctif en ordre inverse
- `git apply --cached chemin/vers/fichier` : stocke le résultat du correctif dans l'index sans modifier la branche courante

## Links
- More information: https://git-scm.com/docs/git-apply
