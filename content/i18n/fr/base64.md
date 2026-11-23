# base64

Encode ou décode un fichier ou l'entrée standard vers ou depuis la base 64, et retourne le résultat à la sortie standard.

## Usage
- `base64 chemin/vers/fichier` : encode un fichier
- `base64 -w 0|76|... chemin/vers/fichier` : enveloppe la sortie codée à une largeur spécifique (`0` désactive l'enveloppement)
- `base64 -d chemin/vers/fichier` : décode un fichier
- `commande | base64` : encode depuis `stdin`
- `commande | base64 -d` : décode depuis `stdin`

## Links
- More information: https://manned.org/base64
