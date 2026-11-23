# aria2c

Utilitaire de téléchargement rapide. Supporte HTTP(S), FTP, SFTP, BitTorrent, et Metalink.

## Usage
- `aria2c "url"` : télécharge depuis une URI spécifique vers un fichier
- `aria2c -o chemin/vers/fichier "url"` : télécharge un fichier via une URI avec un nom de sortie spécifique
- `aria2c -Z "url1" "url2" ...` : télécharge plusieurs fichiers différents en parallèle
- `aria2c --checksum sha-256=hash "url1" "url2" ...` : télécharge le même fichier depuis différents miroirs et vérifie la somme de contrôle du fichier téléchargé
- `aria2c -i chemin/vers/fichier -j nombre_telechargements` : télécharge les URIs listées dans un fichier avec un nombre spécifique de téléchargements en parallèle
- `aria2c -s nombre_connexions "url"` : télécharge avec plusieurs connexions
- `aria2c --ftp-user nom_d_utilisateur --ftp-passwd mot_de_passe "url"` : téléchargement FTP avec nom d'utilisateur et mot de passe
- `aria2c --max-download-limit vitesse "url"` : limite la vitesse de téléchargement en octets/s

## Links
- More information: https://aria2.github.io/manual/en/html/aria2c.html
