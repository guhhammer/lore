# acme.sh

Shell-script dat het ACME-clientprotocol implementeert, een alternatief voor `certbot`.

## Usage
- `acme.sh --issue -d example.com -w /pad/naar/webroot` : geef een certificaat uit met behulp van de webroot-modus
- `acme.sh --issue --standalone -d example.com -d www.example.com` : geef een certificaat uit voor meerdere domeinen in de zelfstandige modus met poort 80
- `acme.sh --issue --alpn -d example.com` : geef een certificaat uit met behulp van de zelfstandige TLS-modus met behulp van poort 443
- `acme.sh --issue --nginx -d example.com` : geef een certificaat uit met een werkende `nginx`-configuratie
- `acme.sh --issue --apache -d example.com` : geef een certificaat uit met een werkende Apache-configuratie
- `acme.sh --issue --dns dns_cf -d *.example.com` : geef een wildcardcertificaat (\*) uit met behulp van een automatische DNS API-modus
- `acme.sh -i -d example.com --key-file /pad/naar/example.com.key --fullchain-file /pad/naar/example.com.cer --reloadcmd "systemctl force-reload nginx"` : installeer certificaatbestanden op de opgegeven locaties (handig voor automatische certificaatvernieuwing)

## Links
- More information: https://github.com/acmesh-official/acme.sh#2-just-issue-a-cert
