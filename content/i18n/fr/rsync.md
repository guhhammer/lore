# rsync

Transférer des fichiers vers ou depuis un hôte distant (pas entre deux hôtes distants). Peut transférer un ou plusieurs fichiers correspondant à un motif.

## Usage
- `rsync chemin/vers/origine chemin/vers/destination` : transfère un fichier
- `rsync -a chemin/vers/origine chemin/vers/destination` : utilise le mode archive (copier récursivement les répertoires, copier les liens symboliques sans résolution et conserver les autorisations, la propriété et les délais de modification)
- `rsync -r chemin/vers/origine chemin/vers/destination` : transférer le contenu d'un dossier
- `rsync -r chemin/vers/origine/ chemin/vers/destination` : transférer le contenu d'un dossier (mais pas le dossier lui-même)
- `rsync -auL chemin/vers/origine chemin/vers/destination` : utiliser le mode archive, résolvant les liens symboliques et ignorant les fichiers déjà transférés sauf si plus récents
- `rsync -r --delete rsync://hote_distant:chemin/vers/origine chemin/vers/destination` : transférer un fichier vers un hôte distant exécutant `rsyncd` et supprimez les fichiers sur la destination qui n'existent pas sur l'hôte distant
- `rsync -e 'ssh -p port' --info=progress2 hote_distant:chemin/vers/origine chemin/vers/destination` : transférer un fichier par SSH et afficher l'avancement global du transfert

## Links
- More information: https://download.samba.org/pub/rsync/rsync.1
