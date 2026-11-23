# git check-ignore

Analyser et déboguer les fichiers ignorés / exclus (`.gitignore`) de Git.

## Usage
- `git check-ignore chemin/vers/fichier_ou_répertoire` : vérifie qu'un fichier ou répertoire est ignoré
- `git check-ignore chemin/vers/fichier_ou_répertoire1 chemin/vers/fichier_ou_répertoire2 ...` : vérifie que plusieurs fichiers ou répertoires sont ignorés
- `git <chemin/vers/fichier_annexe check-ignore --stdin` : utilisez des chemins d'accès, un par ligne, de `stdin`
- `git check-ignore --no-index chemin/vers/fichier_ou_répertoire1 chemin/vers/fichier_ou_répertoire2 ...` : ne pas vérifier l'index (utilisé pour déboguer pourquoi les chemins ont été suivis et non ignorés)
- `git check-ignore -v chemin/vers/fichier_ou_répertoire1 chemin/vers/fichier_ou_répertoire2 ...` : inclure les détails pour chaque occurrence dans le chemin

## Links
- More information: https://git-scm.com/docs/git-check-ignore
