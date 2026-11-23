# ip rule

Gestione del database delle politiche di routing IP.

## Usage
- `ip ru` : mostra la politica di routing
- `sudo ip ru a from all lookup table_id` : crea una nuova regola di routing generica con priorità più alta di `main`
- `sudo ip ru a from 192.168.178.2/32 lookup table_id` : aggiunge una nuova regola basata sugli indirizzi sorgente del pacchetto
- `sudo ip ru a to 192.168.178.2/32 lookup table_id` : aggiunge una nuova regola basata sugli indirizzi di destinazione del pacchetto
- `sudo ip ru d from 192.168.178.2/32` : elimina una regola basata sugli indirizzi sorgente del pacchetto
- `sudo ip ru f` : rimuove tutte le regole di routing
- `ip ru s >path/to/ip_rules.dat` : salva tutte le regole in un file
- `sudo ip <path/to/ip_rules.dat ru r` : ripristina tutte le regole da un file

## Links
- More information: https://manned.org/ip-rule
