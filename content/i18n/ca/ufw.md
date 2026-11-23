# ufw

Tallafocs sense complicacions (Uncomplicated Firewall). Interfície d'usuari de `iptables` per facilitar la configuració d'un firewall.

## Usage
- `sudo ufw enable` : activa ufw
- `sudo ufw disable` : desactiva ufw
- `sudo ufw status numbered` : mostra les regles del ufw, en conjunt amb els seus números
- `sudo ufw allow 5432 comment "servei"` : permet el tràfic entrant en el port 5432 en aquest host amb un comentari que indentifiqui el servei
- `sudo ufw allow proto tcp from 192.168.0.4 to any port 22` : permet només el tràfic TCP desde 192.168.0.4 a qualsevol direcció d'aquest host, en el port 22
- `sudo ufw deny 80` : denega el tràfic en el port 80 en aquest host
- `sudo ufw deny proto udp from any to any port 22` : denega tot el tràfic al port 22
- `sudo ufw delete número_de_regla` : elimina una regla concreta. El número de la regla es pot obtenir del comanadament `ufw status numbered`

## Links
- More information: https://manned.org/ufw
