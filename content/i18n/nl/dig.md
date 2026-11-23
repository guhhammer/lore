# dig

DNS-opzoekhulpprogramma.

## Usage
- `dig +short example.com` : zoek de IP-adres(sen) op die bij een hostnaam horen (A-records)
- `dig +noall +answer example.com` : verkrijg een gedetailleerd antwoord voor een gegeven domein (A-records)
- `dig +short example.com A|MX|TXT|CNAME|NS` : bevraag een specifiek DNS-recordtype voor een gegeven domeinnaam
- `dig +tls @1.1.1.1|8.8.8.8|9.9.9.9|... example.com` : specificeer een alternatieve DNS-server om te bevragen en gebruik optioneel DNS over TLS (DoT)
- `dig -x 8.8.8.8` : voer een reverse DNS-opzoeking uit op een IP-adres (PTR-record)
- `dig +nssearch example.com` : vind autoritatieve naamservers voor de zone en toon SOA-records
- `dig +trace example.com` : voer iteratieve queries uit en toon het volledige tracepad om een domeinnaam te herleiden
- `dig +tcp -p poort @dns_server_ip example.com` : bevraag een DNS-server via een niet-standaard poort met het TCP-protocol

## Links
- More information: https://manned.org/dig
