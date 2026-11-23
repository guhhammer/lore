# b2sum

Calcule les sommes de contrôle cryptographiques BLAKE2.

## Usage
- `b2sum chemin/vers/fichier` : calcule la somme de contrôle BLAKE2 d'un fichier
- `b2sum chemin/vers/fichier1 chemin/vers/fichier2 ... >chemin/vers/fichier.b2` : calcule les sommes de contrôle BLAKE2 pour plusieurs fichiers
- `commande | b2sum` : calcule la somme de contrôle BLAKE2 depuis `stdin`
- `b2sum --check chemin/vers/fichier.b2` : lis un fichier contenant des sommes de contrôle BLAKE2 et des noms de fichier et vérifie que tous les fichiers ont des sommes de contrôle correspondantes
- `b2sum --check --quiet chemin/vers/fichier.b2` : affiche un message uniquement pour les fichiers manquants ou lorsque la vérification échoue
- `b2sum --ignore-missing --check --quiet chemin/vers/fichier.b2` : N'affiche un message que pour les fichiers pour lesquels la vérification a échoué, en ignorant les fichiers manquants
