# ip route get

Ottiene una singola route verso una destinazione e stampa il suo contenuto esattamente come lo vede il kernel.

## Usage
- `ip r g 1.1.1.1` : stampa la route verso una destinazione
- `ip r g destinazione from sorgente` : stampa la route verso una destinazione da un indirizzo sorgente specifico
- `ip r g destinazione iif ethX` : stampa la route verso una destinazione per pacchetti in arrivo su un'interfaccia specifica
- `ip r g destinazione oif ethX` : stampa la route verso una destinazione, forzando l'output attraverso un'interfaccia specifica
- `ip r g destinazione tos 0x10` : stampa la route verso una destinazione con un Type of Service (ToS) specificato
- `ip r g destinazione vrf myvrf` : stampa la route verso una destinazione usando una specifica istanza VRF (Virtual Routing and Forwarding)

## Links
- More information: https://manned.org/ip-route
