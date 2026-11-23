# airshare

Transfère des données entre deux machines dans un réseau local.

## Usage
- `airshare code chemin/vers/fichier_ou_répertoire1 chemin/vers/fichier_ou_répertoire2 ...` : partage des fichiers ou des répertoires
- `airshare code` : reçoit un fichier
- `airshare --upload code` : héberge un serveur de réception (pour pouvoir télécharger des fichiers via l'interface web)
- `airshare --upload code chemin/vers/fichier_ou_répertoire1 chemin/vers/fichier_ou_répertoire2 ...` : envoie des fichiers ou des répertoires a un serveur de reception
- `airshare --file-path code` : envoie les fichiers dont les chemins ont été copiés dans le presse-papiers
- `airshare --clip-receive code` : reçoit un fichier et le copie dans le presse-papier

## Links
- More information: https://airshare.readthedocs.io/en/latest/cli.html
