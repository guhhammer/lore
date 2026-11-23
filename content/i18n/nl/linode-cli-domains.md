# linode-cli domains

Beheer Linode Domains en DNS-configuratie.

## Usage
- `linode-cli domains list` : toon alle beheerde domeinen
- `linode-cli domains create --domain domein_naam --type master|slave --soa-email email` : maak een nieuw beheerd domein
- `linode-cli domains view domein_id` : bekijk details van een specifiek domein
- `linode-cli domains delete domein_id` : verwijder een beheerd domein
- `linode-cli domains records-list domein_id` : toon records voor een specifiek domein
- `linode-cli domains records-create domein_id --type A|AAAA|CNAME|MX|... --name subdomein --target doelwaarde` : voeg een DNS-record toe aan een domein
- `linode-cli domains records-update domein_id record_id --target nieuwe_doelwaarde` : update een DNS-record voor een domein
- `linode-cli domains records-delete domein_id record_id` : verwijder een DNS-record van een domein
