# drill

Esegui varie query DNS.

## Usage
- `drill example.com` : mostra gli IP associati ad un hostname (record A)
- `drill mx example.com` : cerca il/i server di posta associato/i a un dato nome di dominio (record MX)
- `drill any example.com` : recupera tutti i tipi di record per un dato dominio
- `drill example.com @8.8.8.8` : specifica un server DNS alternativo da interrogare
- `drill -x 8.8.8.8` : esegui un lookup DNS inverso su di un indirizzo IP (record PTR)
- `drill -TD example.com` : esegui un tracciamento DNSSEC dai root server fino al dominio
- `drill -s dnskey example.com` : mostra record DNSKEY per un dominio

## Links
- More information: https://manned.org/drill
