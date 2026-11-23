# ip address

Sous-commande de gestion des adresses IP.

## Usage
- `ip a` : liste les interfaces réseau et leurs adresses IP associées
- `ip a s up` : filtre pour n'afficher que les interfaces réseau actives
- `ip a s eth0` : affiche les informations relatives à une interface réseau spécifique
- `sudo ip a a ip_address dev eth0` : ajoute une adresse IP à une interface réseau
- `sudo ip a d ip_address dev eth0` : supprime une adresse réseau d'une interface réseau
- `sudo ip a f eth0 scope global|host|link` : supprime l'ensemble des adresses IP sur une portée donnée (scope) depuis une interface réseau

## Links
- More information: https://manned.org/ip-address
