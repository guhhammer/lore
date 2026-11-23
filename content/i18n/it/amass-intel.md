# amass intel

Raccoglie informazioni open source su un’organizzazione, come domini radice e ASNs.

## Usage
- `amass intel -addr 192.168.0.1-254` : trova i domini radice in un intervallo di indirizzi IP
- `amass intel -active -addr 192.168.0.1-254` : usa metodi di ricognizione attiva
- `amass intel -whois -d nome_dom` : trova i domini radice associati a un dominio
- `amass intel -org nome_org` : trova gli ASNs appartenenti a un’[org]anizzazione
- `amass intel -asn asn` : trova i domini radice appartenenti a un dato Autonomous System Number
- `amass intel -o percorso/del/file_output -whois -d nome_dom` : salva i risultati in un file di testo
- `amass intel -list` : elenca tutte le fonti dati disponibili
