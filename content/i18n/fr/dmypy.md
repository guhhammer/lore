# dmypy

Vérifie les types dans du code Python, en utilisant `mypy` dans un démon pour une exécution plus rapide.

## Usage
- `dmypy run -- chemin/vers/fichier.py` : vérifie les types dans un fichier, et démarre le démon s'il n'est pas lancé
- `dmypy start` : démarre le démon
- `dmypy check -- chemin/vers/fichier.py` : vérifie les types dans un fichier (nécéssite que le démon soit lancé)
- `dmypy stop` : arrête le démon

## Links
- More information: https://mypy.readthedocs.io/en/stable/mypy_daemon.html
