# git am

Applique des fichiers de "correctif" Git et crée une validation. Utile lorsque l'on reçoit des validations par email.

## Usage
- `git am chemin/vers/fichier.patch` : applique et valide un fichier de correctif local
- `curl -L https://example.com/fichier.patch | git am` : applique et valide un fichier de correctif distant
- `git am --abort` : annule l'application d'un fichier de correctif
- `git am --reject chemin/vers/fichier.patch` : applique autant que possible un fichier de correctif, en enregistrant les morceaux qui échouent dans des fichiers de rejet

## Links
- More information: https://git-scm.com/docs/git-am
