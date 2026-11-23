# git describe

Crée un nom unique et lisible pour un objet à partir d'une référence disponible.

## Usage
- `git describe` : crée un nom unique pour la validation actuelle (le nom contient l'étiquette la plus récente, le nombre de validations additionnelles, et l'empreinte abrégée de la validation)
- `git describe --abbrev=4` : crée un nom avec une empreinte de validation de 4 caractères
- `git describe --all` : génère un nom avec le chemin complet de l'étiquette
- `git describe v1.0.0` : décrit une étiquette Git
- `git describe nom_branche` : crée un nom pour la dernière validation d'une branche donnée

## Links
- More information: https://git-scm.com/docs/git-describe
