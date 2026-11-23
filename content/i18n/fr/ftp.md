# ftp

Outils permettant d'interagir avec un serveur avec le protocole FTP.

## Usage
- `ftp ftp.example.com` : connexion à un serveur FTP
- `ftp adresse_ip port` : connexion à un serveur FTP en spécifiant son adresse IP et son port
- `binary` : [Interactif] Passe en mode de transfert binaire (médias, fichiers compressés, etc)
- `prompt off` : [Interactif] Transfère plusieurs fichiers sans demander de confirmation pour chaque
- `mget *.png` : [Interactif] Télécharge plusieurs fichiers
- `mput *.zip` : [Interactif] Téléverse plusieurs fichiers
- `mdelete *.txt` : [Interactif] Supprime plusieurs fichiers sur le serveur distant
- `rename ancien_fichier nouveau_fichier` : [Interactif] Renomme un fichier sur le serveur distant

## Links
- More information: https://manned.org/ftp
