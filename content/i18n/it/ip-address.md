# ip address

Sottocomando di gestione degli indirizzi IP.

## Usage
- `ip a` : elenca le interfacce di rete e i loro indirizzi IP associati
- `ip a s up` : filtra per mostrare solo le interfacce di rete attive
- `ip a s ethX` : mostra informazioni su un'interfaccia di rete specifica
- `sudo ip a a ip_address dev ethX` : aggiunge un indirizzo IP a un'interfaccia di rete
- `sudo ip a d ip_address dev ethX` : rimuove un indirizzo IP da un'interfaccia di rete
- `sudo ip a f ethX scope global|host|link` : elimina tutti gli indirizzi IP in un dato ambito da un'interfaccia di rete

## Links
- More information: https://manned.org/ip-address
