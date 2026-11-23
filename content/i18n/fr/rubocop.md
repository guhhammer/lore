# rubocop

Lint les fichiers Ruby.

## Usage
- `rubocop` : vérifie tous les fichiers du répertoire actuel (y compris les sous-répertoires)
- `rubocop chemin/vers/fichier_ou_dossier1 chemin/vers/fichier_ou_dossier2 ...` : vérifie un ou plusieurs fichiers ou répertoires spécifiques
- `rubocop --out chemin/vers/fichier` : écrit la sortie dans un fichier
- `rubocop --show-cops` : affiche la liste des cops (règles de lint)
- `rubocop --except cop_1 cop_2 ...` : exclue un cop
- `rubocop --only cop_1 cop_2 ...` : exécute uniquement les cops spécifiés
- `rubocop --auto-correct` : corrige automatiquement les fichiers (expérimental)

## Links
- More information: https://docs.rubocop.org/rubocop/latest/usage/cli_reference.html
