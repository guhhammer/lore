# arping

Scopri e interroga host in una rete utilizzando il protocollo ARP. Utile per scoprire indirizzi MAC.

## Usage
- `sudo arping ip_host` : invia un ping ad un host inviando pacchetti ARP request
- `sudo arping -I interfaccia ip_host` : invia un pint ad un host su una specifica interfaccia
- `sudo arping -f ip_host` : invia un ping ad un host e termina all prima risposta
- `sudo arping -c count ip_host` : invia uno specifico numero di ping
- `sudo arping -U ip_da_inviare_in_broadcast` : invia pacchetti ARP request in broadcast per aggiornare la cache ARP dei vicini
- `sudo arping -D -w 3 ip_da_controllare` : rileva indirizzi IP duplicati nella rete inviando richieste ARP con un timeout di 3 secondi

## Links
- More information: https://manned.org/arping
