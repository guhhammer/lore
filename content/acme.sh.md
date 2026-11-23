# acme.sh

Shell script implementing ACME client protocol, an alternative to `certbot`.

## Usage
- `acme.sh --issue -d example.com -w /path/to/webroot` : issue a certificate using webroot mode
- `acme.sh --issue --standalone -d example.com -d www.example.com` : issue a certificate for multiple domains using standalone mode using port 80
- `acme.sh --issue --alpn -d example.com` : issue a certificate using standalone TLS mode using port 443
- `acme.sh --issue --nginx -d example.com` : issue a certificate using a working `nginx` configuration
- `acme.sh --issue --apache -d example.com` : issue a certificate using a working Apache configuration
- `acme.sh --issue --dns dns_cf -d *.example.com` : issue a wildcard (\*) certificate using an automatic DNS API mode
- `acme.sh -i -d example.com --key-file /path/to/example.com.key --fullchain-file /path/to/example.com.cer --reloadcmd "systemctl force-reload nginx"` : install certificate files into the specified locations (useful for automatic certificate renewal)

## See also
- acme.sh dns

## Categories
security, network, files

## Links
- More information: https://github.com/acmesh-official/acme.sh#2-just-issue-a-cert
- tldr page: https://tldr.inbrowser.app/pages/common/acme.sh
