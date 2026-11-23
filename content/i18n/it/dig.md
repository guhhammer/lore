# dig

Utilità di lookup DNS.

## Usage
- `dig +short example.com` : mostra gli IP associati ad un hostname (record A)
- `dig +short example.com MX` : mostra i mail server associati ad uno specifico dominio (record MX)
- `dig @8.8.8.8 example.com` : specifica un server DNS alternativo a cui fare richiesta
- `dig -x 8.8.8.8` : esegui un lookup DNS inverso su di un indirizzo IP (record PTR)
- `dig +nssearch example.com` : trova i nameserver autoritativi per la zona e mostra i record SOA
- `dig +trace example.com` : esegui richieste iterative e mostra l'intero percorso per risolvere il dominio

## Links
- More information: https://manned.org/dig
