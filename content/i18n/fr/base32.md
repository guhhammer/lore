# base32

Encode ou décode un fichier ou l'entrée standard vers ou depuis la base 32, et retourne le résultat à la sortie standard.

## Usage
- `base32 chemin/vers/fichier` : encode un fichier
- `base32 -w 0|76|... chemin/vers/fichier` : enveloppe la sortie codée à une largeur spécifique (`0` désactive l'enveloppement)
- `base32 -d chemin/vers/fichier` : décode un fichier
- `commande | base32` : encode depuis `stdin`
- `commande | base32 -d` : décode depuis `stdin`

## Links
- More information: https://manned.org/base32
