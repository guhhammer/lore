# pfctl

Contrôle le dispositif de filtrage de paquets.

## Usage
- `sudo pfctl -e` : active le filtre de paquets
- `sudo pfctl -d` : désactive le filtre de paquets
- `sudo pfctl -f chemin/vers/pf.conf` : charge les règles depuis un fichier de configuration
- `pfctl -sr` : affiche toutes les règles actives
- `pfctl -s info` : affiche les informations d’état du filtre de paquets
- `sudo pfctl -F rules` : supprime toutes les règles

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=pfctl
