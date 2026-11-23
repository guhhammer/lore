# ar

Crée, modifie et extrait depuis des archives (`.a`, `.so`, `.o`).

## Usage
- `ar x chemin/vers/archive.a` : extrait tous les éléments depuis une archive
- `ar t chemin/vers/archive.ar` : liste tous les éléments depuis une archive
- `ar r chemin/vers/archive.deb chemin/vers/debian-binary chemin/vers/control.tar.gz chemin/vers/data.tar.xz ...` : remplace ou ajoute des fichiers à une archive
- `ar s chemin/vers/archive.a` : insère un fichier d'indexation (équivalent à `ranlib`)
- `ar rs chemin/vers/archive.a chemin/vers/fichier1.o chemin/vers/fichier2.o ...` : crée une archive avec des fichiers et un fichier d'indexation qui l'accompagne

## Links
- More information: https://manned.org/ar
