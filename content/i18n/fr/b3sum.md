# b3sum

Calcule les sommes de contrôle cryptographiques BLAKE3.

## Usage
- `b3sum chemin/vers/fichier1 chemin/vers/fichier2 ...` : calcule la somme de contrôle BLAKE3 pour un ou plusieurs fichiers
- `b3sum chemin/vers/fichier1 chemin/vers/fichier2 ... >chemin/vers/fichier.b3` : calcule et enregistre la liste des sommes de contrôle BLAKE3 dans un fichier
- `commande | b3sum` : calculer une somme de contrôle BLAKE3 à partir de `stdin`
- `b3sum --check chemin/vers/fichier.b3` : lit un fichier de sommes et de noms de fichiers BLAKE3 et vérifie que tous les fichiers ont des sommes de contrôle correspondantes
- `b3sum --check --quiet chemin/vers/fichier.b3` : N'affiche un message que pour les fichiers manquants ou en cas d'échec de la vérification

## Links
- More information: https://github.com/BLAKE3-team/BLAKE3/tree/master/b3sum
