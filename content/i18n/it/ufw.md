# ufw

Ufw (Uncomplicated Firewall) - Firewall Semplice. Frontend per `iptables` per semplificare la configurazione di un firewall.

## Usage
- `sudo ufw enable` : accendi ufw
- `sudo ufw disable` : spegni ufw
- `sudo ufw status numbered` : mostra le regole di ufw, con i numeri corrispondenti
- `sudo ufw allow 5432 comment "servizio"` : aperto al traffico in entrata sulla porta 5432, con un commento che identifica il servizio
- `sudo ufw allow proto tcp from 192.168.0.4 to any port 22` : aperto solo al traffico TCP da 192.168.0.4 a qualsiasi indirizzo su questo host, sulla porta 22
- `sudo ufw deny 80` : blocchi traffico su porta 80 su questo host
- `sudo ufw deny proto udp from any to any port 22` : nega tutto il traffico UDP alla porta 22
- `sudo ufw delete numero_della_regola` : elimina una regola particolare. Il numero della regola può essere trovato con "ufw status numbered"

## Links
- More information: https://manned.org/ufw
