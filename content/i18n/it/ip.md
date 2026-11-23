# ip

Mostra / manipola routing, dispositivi, criteri di routing e tunnel. Alcuni sottocomandi, come `address`, hanno una propria documentazione d'uso.

## Usage
- `ip a` : elenca le interfacce con informazioni dettagliate
- `ip -br a` : elenca le interfacce con informazioni brevi sul livello di rete
- `ip -br l` : elenca le interfacce con informazioni brevi sul livello di collegamento
- `ip r` : visualizza la tabella di routing
- `ip n` : mostra i vicini (tabella ARP)
- `sudo ip l s interfaccia up|down` : attiva/disattiva un'interfaccia
- `sudo ip a add|delete ip_address/mask dev interfaccia` : aggiungi/elimina un indirizzo IP a/da un'interfaccia
- `sudo ip r a default via ip_address dev interfaccia` : aggiungi una route predefinita

## Links
- More information: https://manned.org/ip.8
