# nslookup

Bevraag naamservers voor verschillende domeinrecords.

## Usage
- `nslookup example.com` : bevraag de standaard naamserver van je systeem voor een IP-adres (A-record) van het domein
- `nslookup -type=NS example.com 8.8.8.8` : bevraag een gegeven naamserver voor een NS-record van het domein
- `nslookup -type=PTR 54.240.162.118` : bevraag een reverse lookup (PTR-record) van een IP-adres
- `nslookup -vc -type=ANY example.com` : bevraag alle beschikbare records (type ANY) met het TCP-protocol
- `nslookup -vc -type=AXFR example.com naamserver` : bevraag een gegeven naamserver voor het volledige zonebestand (zone transfer) van het domein met het TCP-protocol
- `nslookup -type=MX -debug example.com` : bevraag een mailserver (MX-record) van het domein, met details van de transactie
- `nslookup -port=poortnummer -type=TXT example.com naamserver` : bevraag een gegeven naamserver op een specifiek poortnummer voor een TXT-record van het domein

## Links
- More information: https://manned.org/nslookup
