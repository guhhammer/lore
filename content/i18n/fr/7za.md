# 7za

Un archiveur de fichiers avec un haut taux de compression. Similaire à `7z` sauf qu'il supporte moins de type de fichiers mais est multi-plateforme.

## Usage
- `7za a chemin/vers/archive.7z chemin/vers/fichier_ou_répertoire` : compresse un fichier ou un répertoire
- `7za a chemin/vers/archive_chiffree.7z -pmotdepasse -mhe=on chemin/vers/archive.7z` : chiffre une archive existante (en incluant les en-têtes)
- `7za x chemin/vers/archive.7z` : extrait une archive en conservant l'arborescence des fichiers
- `7za x chemin/vers/archive.7z -ochemin/vers/sortie` : extrait une archive vers un dossier specifique
- `7za x chemin/vers/archive.7z -so` : extrait une archive vers sortie standard
- `7za a -t7z|bzip2|gzip|lzip|tar|... chemin/vers/archive.7z chemin/vers/fichier_ou_répertoire` : compresse en utilisant une compression spécifique
- `7za l chemin/vers/archive.7z` : liste le contenu d'une archive
- `7za a chemin/vers/archive.7z -mx=0|1|3|5|7|9 chemin/vers/fichier_ou_répertoire` : définit le niveau de compression (plus il est élevé, plus la compression est importante, mais plus elle est lente)

## Links
- More information: https://manned.org/7za
