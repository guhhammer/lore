# exiftool

Lit et écrit les métadonnées dans des fichiers.

## Usage
- `exiftool chemin/vers/fichier` : affiche les métadonnées EXIF d'un fichier donné
- `exiftool -All= chemin/vers/fichier1 chemin/vers/fichier2 ...` : supprime toutes les métadonnées EXIF des fichiers donnés
- `exiftool "-gps*=" chemin/vers/image1 chemin/vers/image2 ...` : supprime les métadonnées EXIF GPS des fichiers image donnés
- `exiftool -All= -tagsfromfile @ -colorspacetags -orientation chemin/vers/image1 chemin/vers/image2 ...` : supprime toutes les métadonnées EXIF des fichiers image donnés, puis ré-ajoute les métadonnées de couleur et d'orientation
- `exiftool "-AllDates+=0:0:0 1:0:0" chemin/vers/répertoire` : avance d'une heure la date de prise de toutes les photos d'un répertoire
- `exiftool "-AllDates-=0:0:1 2:0:0" -ext jpg` : recule d'un jour et deux heures la date de prise de toutes les photos JPEG du répertoire actuel
- `exiftool -DateTimeOriginal-=1.5 -overwrite_original` : modifie uniquement le champ `DateTimeOriginal` en soustrayant 1,5 heure, sans conserver de sauvegardes
- `exiftool '-filename<DateTimeOriginal' -d %Y-%m-%d_%H-%M-%S%%lc.%%e chemin/vers/répertoire -r -ext jpg` : renomme récursivement toutes les photos JPEG d'un répertoire à partir du champ `DateTimeOriginal`

## Links
- More information: https://exiftool.org/exiftool_pod.html
