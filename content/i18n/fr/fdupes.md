# fdupes

Trouve les fichiers dupliqués dans les répertoires donnés.

## Usage
- `fdupes chemin/vers/répertoire` : cherche dans un répertoire
- `fdupes chemin/vers/répertoire1 chemin/vers/répertoire2 ...` : cherche dans plusieurs répertoires
- `fdupes -r chemin/vers/répertoire` : cherche dans un répertoire récursivement
- `fdupes chemin/vers/répertoire1 -R chemin/vers/répertoire2` : cherche dans plusieurs répertoires dont un récursivement
- `fdupes -rH chemin/vers/répertoire` : cherche récursivement, en considérant les liens physiques comme des doublons
- `fdupes -rd chemin/vers/répertoire` : cherche récursivement les dupliqués et demande les fichiers à conserver, supprimant les autres
- `fdupes -rdN chemin/vers/répertoire` : cherche récursivement et supprime les dupliqués automatiquement

## Links
- More information: https://github.com/adrianlopezroche/fdupes#introduction
