# zip

Empaquette et compresse (archive) les fichiers en un fichier zip.

## Usage
- `zip -r archive.zip chemin/du/répertoire` : empaquette et compresse récursivement un répertoire et son contenu
- `zip -r archive.zip chemin/vers/le/répertoire -x chemin/des/fichiers/exclus` : exclure des fichiers de l'archive
- `zip -r -9 archive.zip chemin/du/répertoire` : archive un répertoire et son contenu avec le plus haut niveau [9] de compression
- `zip -r archive.zip chemin/du/répertoire1 chemin/du/répertoire2 chemin/du/fichier` : empaquette et compresse plusieurs répertoires et fichiers
- `zip -re archive.zip chemin/du/répertoire` : crée une archive chiffrée (l'utilisateur sera sollicité pour saisir le mot de passe)
- `zip archive.zip chemin/du/fichier` : ajoute des fichiers à une archive existante
- `zip -d archive.zip "foo/*.tmp"` : supprime des fichiers d'une archive existante
- `zip -rs 3g archive.zip chemin/du/répertoire` : archive un répertoire et son contenu en plusieurs fichiers zip scindés (ex : des fichiers de 3 Go)

## Links
- More information: https://manned.org/zip
