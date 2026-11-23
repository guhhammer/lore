# ip

Ukazuje/nastavuje routování, zařízení, pravidla routování a tunely. Některé dílčí příkazy jako je `address` mají svou vlastní dokumentaci.

## Usage
- `ip a` : vypsat rozhraní s podrobnými informacemi
- `ip -br a` : vypsat rozhraní se stručnými informacemi o síťové vrstvě
- `ip -br l` : vypsat rozhraní se stručnými informacemi o linkové vrstvě
- `ip r` : zobrazit routovací tabulku
- `ip n` : ukázat sousedy (ARP tabulka)
- `sudo ip l s ethX up|down` : změnit rozhraní nahoru/dolů
- `sudo ip a add|delete ip_address/maska dev ethX` : přidat/Smazat IP adresu k rozhraní
- `sudo ip r a default via ip_address dev ethX` : přidat výchozí cestu

## Links
- More information: https://manned.org/ip.8
