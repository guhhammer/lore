# acme.sh-dns

Gebruik een DNS-01 challenge om een TLS-certificaat uit te geven.

## Usage
- `acme.sh --issue --dns dns_gd --domain example.com` : geef een certificaat uit met behulp van een automatische DNS API-modus
- `acme.sh --issue --dns dns_namesilo --domain example.com --domain *.example.com` : geef een wildcardcertificaat uit (aangegeven met een asterisk) met behulp van een automatische DNS API-modus
- `acme.sh --issue --dns dns_cf --domain example.com --challenge-alias alias-voor-voorbeeld-validatie.com` : geef een certificaat uit met behulp van een DNS-aliasmodus
- `acme.sh --issue --dns dns_namecheap --domain example.com --dnssleep 300` : geef een certificaat uit terwijl u automatische Cloudflare / Google DNS-polling uitschakelt nadat het DNS-record is toegevoegd door een aangepaste wachttijd in seconden op te geven
- `acme.sh --issue --dns --domain example.com --yes-I-know-dns-manual-mode-enough-go-ahead-please` : geef een certificaat uit met behulp van een handmatige DNS-modus

## Links
- More information: https://github.com/acmesh-official/acme.sh/wiki
