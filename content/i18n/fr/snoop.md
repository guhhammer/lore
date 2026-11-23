# snoop

Renifleur de paquets réseau. Équivalent SunOS de `tcpdump`.

## Usage
- `snoop -d e1000g0` : capture des paquets sur une interface réseau spécifique
- `snoop -o chemin/vers/fichier` : enregistre les paquets capturés dans un fichier au lieu de les afficher
- `snoop -V -i chemin/vers/fichier` : affiche le résumé détaillé de la couche de protocole des paquets d'un fichier
- `snoop to port port from host nom_d'hôte` : capture les paquets réseau provenant d'un nom d'hôte et accède à un port donné
- `snoop -x0 -p4 adresse_ip_1 adresse_ip_2` : capture et affiche un vidage hexadécimal des paquets réseau échangés entre deux adresses IP

## Links
- More information: https://www.unix.com/man-page/sunos/1m/snoop
