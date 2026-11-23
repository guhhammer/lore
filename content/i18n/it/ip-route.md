# ip route

Sottocomando di gestione della tabella di routing IP.

## Usage
- `ip r` : mostra la tabella di routing `main`
- `sudo ip r a default via ip_gateway` : aggiunge una route predefinita usando l'inoltro gateway
- `sudo ip r a default dev ethX` : aggiunge una route predefinita usando `ethX`
- `sudo ip r a ip_destinazione via ip_gateway dev ethX` : aggiunge una route statica
- `sudo ip r d ip_destinazione dev ethX` : elimina una route statica
- `sudo ip r change|replace ip_destinazione via ip_gateway dev ethX` : modifica o sostituisce una route statica
- `ip r g ip_destinazione` : mostra quale route verrà usata dal kernel per raggiungere un indirizzo IP
- `ip r l t numero_table` : mostra una tabella di routing specifica

## Links
- More information: https://manned.org/ip-route
