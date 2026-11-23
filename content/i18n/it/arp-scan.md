# arp-scan

Invia pacchetti ARP agli host (IP o hostname) per scansionare la rete locale.

## Usage
- `arp-scan -l` : scansiona la rete locale corrente
- `arp-scan 10.0.0.1` : scansiona un host specifico
- `arp-scan 192.168.1.1/24` : scansiona una rete IP con maschera personalizzata
- `arp-scan 127.0.0.0-127.0.0.31` : scansiona una rete IP in un intervallo personalizzato
- `arp-scan 10.0.0.0:255.255.255.0` : scansiona una rete IP con maschera di sottorete personalizzata
- `arp-scan 10.0.0.1 -I nome_interfaccia` : specifica l'interfaccia di rete da usare per la scansione

## Links
- More information: https://github.com/royhills/arp-scan
