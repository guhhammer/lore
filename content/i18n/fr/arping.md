# arping

Découvre et sonde des hôtes dans un réseau en utilisant le protocol ARP. Très utile pour la découverte d'adresse MAC.

## Usage
- `sudo arping ip_hôte` : ping un hôte par paquets de requête ARP
- `sudo arping -I interface ip_hôte` : ping un hôte sur une interface spécifique
- `sudo arping -f ip_hôte` : ping un hôte et arrête à la première réponse
- `sudo arping -c nombre_d_appel ip_hôte` : ping un hôte un certain nombre de fois
- `sudo arping -U ip_à_diffuser` : diffuse les paquets de requête ARP pour mettre à jour les caches ARP voisin
- `sudo arping -D -w 3 ip_à_vérifier` : détecte les adresses IP dupliquées dans le réseau en envoyant les requêtes ARP avec une expiration de 3 secondes

## Links
- More information: https://manned.org/arping
