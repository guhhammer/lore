# tar

Utilitaire d'archivage. Souvent combiné avec une méthode de compression, telle que gzip ou bzip2.

## Usage
- `tar cf chemin/vers/cible.tar chemin/vers/fichier1 chemin/vers/fichier2 ...` : crée une archive à partir de fichiers
- `tar czf chemin/vers/cible.tar.gz chemin/vers/fichier1 chemin/vers/fichier2 ...` : crée une archive gzip à partir de fichiers
- `tar czf chemin/vers/cible.tar.gz -C chemin/vers/répertoire .` : crée une archive gzip à partir d'un répertoire en utilisant son chemin relatif
- `tar xvf chemin/vers/source.tar.ext` : extrait une archive (compressée) dans le répertoire actuel en affichant la liste des fichiers traités
- `tar xf chemin/vers/source.tar.ext -C répertoire` : extrait une archive (compressée) dans un répertoire cible
- `tar caf chemin/vers/cible.tar.xz chemin/vers/fichier1 chemin/vers/fichier2 ...` : crée une archive compressée, en utilisant le suffixe de l'archive pour déterminer le programme de compression
- `tar tvf chemin/vers/source.tar` : liste les fichiers contenus dans une archive tar
- `tar xf source.tar --wildcards "*.html"` : extrait les fichiers correspondant au motif

## Links
- More information: https://www.gnu.org/software/tar/manual/tar.html
