# ip route add

Aggiunge una nuova route di rete.

## Usage
- `sudo ip r a default via ip_gateway` : aggiunge una route predefinita usando l'inoltro gateway
- `sudo ip r a default dev ethX` : aggiunge una route predefinita usando `ethX`
- `sudo ip r a ip_destinazione via ip_gateway dev ethX` : aggiunge una route statica
- `sudo ip r a ip_destinazione dev ethX t indirizzo_ip` : aggiunge una route a una tabella di routing specifica
- `sudo ip r a default via ip_gateway dev ethX table id_table` : forza il traffico internet attraverso un dispositivo specifico lasciando invariato il traffico locale (richiede anche la creazione di una regola)

## Links
- More information: https://manned.org/ip-route
