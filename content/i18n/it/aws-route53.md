# aws route53

CLI per AWS Route53 - Route 53 è un servizio web DNS (Domain Name System) altamente disponibile e scalabile.

## Usage
- `aws route53 list-hosted-zones` : elenca tutte le zone ospitate, private e pubbliche
- `aws route53 list-resource-record-sets --hosted-zone-id id_zona` : mostra tutti i record in una zona
- `aws route53 create-hosted-zone --name nome --caller-reference identificativo_richiesta` : crea una nuova zona pubblica utilizzando un identificativo di richiesta per ritentare l'operazione in modo sicuro
- `aws route53 delete-hosted-zone --id id_zona` : elimina una zona (se la zona ha record SOA e NS non predefiniti il comando fallirà)
- `aws route53 test-dns-answer --hosted-zone-id id_zona --record-name nome --record-type tipo` : testa la risoluzione DNS da parte dei server Amazon di una zona data

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/route53/
