# ip link

Gestisce le interfacce di rete.

## Usage
- `ip l` : mostra informazioni su tutte le interfacce di rete
- `ip l sh ethX` : mostra informazioni su un'interfaccia di rete specifica
- `sudo ip l s ethX up|down` : attiva o disattiva un'interfaccia di rete
- `sudo ip l s ethX al "LAN Interface"` : assegna un nome significativo a un'interfaccia di rete
- `sudo ip l s ethX a ff:ff:ff:ff:ff:ff` : cambia l'indirizzo MAC di un'interfaccia di rete
- `sudo ip l s ethX mtu 9000` : cambia la dimensione MTU per un'interfaccia di rete per usare jumbo frame
- `sudo ip l s ethX promisc on|off` : imposta lo stato della modalità promiscua di un dispositivo
- `sudo ip l d ethX` : elimina un dispositivo

## Links
- More information: https://manned.org/ip-link
