# cidr

Semplifica la gestione dei prefissi di rete IPv4/IPv6 CIDR con conteggio, controllo sovrapposizioni, spiegazioni e suddivisione.

## Usage
- `cidr explain 10.0.0.0/16` : spiega un intervallo CIDR
- `cidr contains 10.0.0.0/16 10.0.14.5` : controlla se un indirizzo appartiene a un intervallo CIDR
- `cidr count 10.0.0.0/16` : conta tutti gli indirizzi in un intervallo CIDR
- `cidr overlaps 10.0.0.0/16 10.0.14.0/22` : controlla se due intervalli CIDR si sovrappongono
- `cidr divide 10.0.0.0/16 9` : suddividi un intervallo CIDR in un numero specifico di reti

## Links
- More information: https://github.com/bschaatsbergen/cidr
