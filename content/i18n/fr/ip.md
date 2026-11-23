# ip

Affiche / manipule l'adressage, le routage, les interfaces et périphériques réseau, les règles de routage et les tunnels. Certaines commandes comme `address` ont leur propre documentation.

## Usage
- `ip a` : liste les interfaces avec des infos détaillées
- `ip -br a` : liste les interfaces sur la couche réseau de façon synthétique
- `ip -br l` : liste les interfaces sur la couche liaison de façon synthétique
- `ip r` : affiche la table de routage
- `ip n` : affiche les voisins (table ARP)
- `sudo ip l s interface up|down` : active/Désactive une interface
- `sudo ip a add|delete adresse_ip/masque dev interface` : ajoute/Supprime une adresse IP à une interface
- `sudo ip r a default via adresse_ip dev interface` : ajoute une route par défaut

## Links
- More information: https://manned.org/ip.8
