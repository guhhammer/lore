# du

Utilisation de disque : estime et résume l'utilisation de l'espace occupé par les fichiers et les répertoires.

## Usage
- `du -b|k|m chemin/vers/répertoire` : liste les tailles d'un répertoire et de ses sous-répertoires, dans l'unité donnée (o/Kio/Mio)
- `du -h chemin/vers/répertoire` : liste les tailles d'un répertoire et de ses sous-répertoires, sous une forme lisible par l'homme (c'est-à-dire en sélectionnant automatiquement l'unité appropriée pour chaque taille)
- `du -sh chemin/vers/répertoire` : affiche la taille d'un répertoire unique, en unités lisibles par l'homme
- `du -ah chemin/vers/répertoire` : liste les tailles, en unités lisibles par l'homme, d'un répertoire et de tous les fichiers et répertoires qu'il contient
- `du -h -d n chemin/vers/répertoire` : liste les tailles, en unités lisibles par l'homme, d'un répertoire et de ses sous-répertoires, jusqu'à `n` niveaux de profondeur
- `du -ch *.jpg` : liste la taille, en unités lisibles par l'homme, de tous les fichiers `.jpg` dans le répertoire actuel, et affiche un total cumulatif à la fin
- `du -ah -t 1G|1024M|1048576K .[^.]* *` : liste tous les fichiers et répertoires (y compris les fichiers cachés) dépassant un certain seuil (utile pour déterminer ce qui occupe réellement l'espace)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/du-invocation.html
