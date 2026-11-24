# nslookup

Query name servers for various domain records.

## Usage
- `nslookup example.com` : query your system's default name server for an IP address (A record) of the domain
- `nslookup -type=NS example.com 8.8.8.8` : query a given name server for an NS record of the domain
- `nslookup -type=PTR 54.240.162.118` : query for a reverse lookup (PTR record) of an IP address
- `nslookup -vc -type=ANY example.com` : query for ANY available records using TCP protocol
- `nslookup -vc -type=AXFR example.com name_server` : query a given name server for the whole zone file (zone transfer) of the domain using TCP protocol
- `nslookup -type=MX -debug example.com` : query for a mail server (MX record) of the domain, showing details of the transaction
- `nslookup -port=port_number -type=TXT example.com name_server` : query a given name server on a specific port number for a TXT record of the domain

## See also
- dig
- resolvectl
- host

## Categories
network

## Links
- More information: https://manned.org/nslookup
- tldr page: https://tldr.inbrowser.app/pages/common/nslookup
- Wikipedia: https://en.wikipedia.org/wiki/Nslookup
