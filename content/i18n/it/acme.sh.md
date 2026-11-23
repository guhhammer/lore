# acme.sh

Script shell che implementa il protocollo client ACME, alternativa a `certbot`.

## Usage
- `acme.sh --issue -d example.com -w //percorso/alla/webroot` : emetti un certificato usando la modalità webroot
- `acme.sh --issue --standalone -d example.com -d www.example.com` : emetti un certificato per più domini usando la modalità standalone sulla porta 80
- `acme.sh --issue --alpn -d example.com` : emetti un certificato usando la modalità standalone TLS sulla porta 443
- `acme.sh --issue --nginx -d example.com` : emetti un certificato usando una configurazione `nginx` funzionante
- `acme.sh --issue --apache -d example.com` : emetti un certificato usando una configurazione Apache funzionante
- `acme.sh --issue --dns dns_cf -d *.example.com` : emetti un certificato wildcard (\*) usando una modalità DNS API automatica
- `acme.sh -i -d example.com --key-file //percorso/a/example.com.key --fullchain-file //percorso/a/example.com.cer --reloadcmd "systemctl force-reload nginx"` : installa i file di certificato nelle posizioni specificate (utile per il rinnovo automatico)

## Links
- More information: https://github.com/acmesh-official/acme.sh#2-just-issue-a-cert
