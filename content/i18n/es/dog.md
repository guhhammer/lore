# dog

Utilidad de búsqueda DNS. Tiene una salida colorida, es compatible con los protocolos DNS sobre TLS y DNS sobre https, y puede emitir JSON.

## Usage
- `dog example.com` : busca las direcciones IP asociadas a un nombre de host (registros A)
- `dog example.com MX` : consulta el tipo de registros MX asociados a un nombre de dominio determinado
- `dog example.com MX @1.1.1.1` : especifica un servidor DNS concreto para la consulta (por ejemplo, Cloudflare)
- `dog example.com MX @1.1.1.1 -T` : consulta a través de TCP en lugar de UDP
- `dog -q example.com -t MX -n 1.1.1.1 -T` : consulta el tipo de registros MX asociados a un nombre de dominio determinado a través de TCP utilizando argumentos explícitos
- `dog example.com -H @https://cloudflare-dns.com/dns-query` : busca las direcciones IP asociadas a un nombre de host (registros A) utilizando DNS sobre https (DoH)

## Links
- More information: https://github.com/ogham/dog#examples
