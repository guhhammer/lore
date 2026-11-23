# cp

Copie des fichiers et des répertoires.

## Usage
- `cp chemin/vers/fichier_source.ext chemin/vers/fichier_cible.ext` : copier un fichier vers un autre emplacement
- `cp chemin/vers/fichier_source.ext chemin/vers/répertoire_parent_cible` : copier un fichier vers un autre répertoire en conservant le nom du fichier
- `cp -R chemin/vers/répertoire_source chemin/vers/répertoire_cible` : copier récursivement le contenu d'un répertoire vers un autre emplacement (si la destination existe, le répertoire est copié à l'intérieur)
- `cp -vR chemin/vers/répertoire_source chemin/vers/répertoire_cible` : copier un répertoire récursivement, en mode verbeux (affiche les fichiers au fur et à mesure de leur copie)
- `cp -i *.txt chemin/vers/répertoire_cible` : copier les fichiers texte vers un autre emplacement, en mode interactif (demande confirmation avant d'écraser)
- `cp -L link chemin/vers/répertoire_cible` : déréférencer les liens symboliques avant de copier

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
