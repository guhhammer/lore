# assimp

Client en ligne de commande pour l'Open Asset Import Library. Supporte le chargement de plus de 40 formats de fichiers 3D, et exporte vers quelques formats 3D populaire.

## Usage
- `assimp listext` : liste tous les formats d'import supportés
- `assimp listexport` : liste tous les formats de sortie supportés
- `assimp export fichier_d_entrée.stl fichier_de_sortie.obj` : convertit un fichier vers un format de sortie supporté, avec les paramètres par défaut
- `assimp export fichier_d_entrée.stl fichier_de_sortie.obj paramètres` : convertit un fichier avec des paramètres personnalisés (le fichier `dox_cmd.h` dans le code source de assimp liste tous les paramètres disponible)
- `assimp info chemin/vers/fichier` : affiche un sommaire du contenu d'un fichier 3D
- `assimp help` : affiche l'aide
- `assimp sous_commande --help` : affiche l'aide d'un sous-commande (e.g les paramètres qui lui sont spécifique)

## Links
- More information: https://manned.org/assimp
