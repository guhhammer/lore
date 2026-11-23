# ip maddress

Gestisce gli indirizzi multicast.

## Usage
- `ip m` : elenca gli indirizzi multicast e quanti programmi vi sono iscritti
- `ip m s dev ethX` : elenca gli indirizzi specifici del dispositivo
- `sudo ip m a 33:33:00:00:00:02 dev ethX` : si unisce staticamente a un gruppo multicast
- `sudo ip m d 33:33:00:00:00:02 dev ethX` : abbandona un gruppo multicast statico
- `ip m h` : mostra l'aiuto

## Links
- More information: https://manned.org/ip-maddress
