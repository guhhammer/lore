# mypy

Vérifie les types dans un code Python.

## Usage
- `mypy chemin/vers/fichier.py` : vérifie les types pour un fichier spécifique
- `mypy -m nom_module` : vérifie les types pour un module
- `mypy -p nom_paquet` : vérifie les types pour un paquet
- `mypy -c "code"` : vérifie les types pour une chaîne de code
- `mypy --ignore-missing-imports chemin/vers/fichier_ou_répertoire` : ignore les imports manquants
- `mypy --tb chemin/vers/fichier_ou_répertoire` : montre le détail des messages d'erreurs
- `mypy --config-file chemin/vers/fichier_de_configuration` : spécifie un fichier de configuration
- `mypy -h` : affiche l'aide

## Links
- More information: https://mypy.readthedocs.io/en/stable/running_mypy.html
