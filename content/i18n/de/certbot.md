# certbot

The Let's Encrypt Agent zum automatischen Erhalten und Erneuern von TLS-Zertifikaten. Nachfolger von `letsencrypt`.

## Usage
- `sudo certbot certonly --webroot -w pfad/zu/webroot -d subdomain.example.com` : beziehe ein neues Zertifikat über die webroot-Autorisierung, aber ohne dieses automatisch zu installieren
- `sudo certbot --nginx -d subdomain.example.com` : beziehe ein neues Zertifikat über die `nginx`-Autorisierung und automatische Installation des neuen Zertifikats
- `sudo certbot --apache -d subdomain.example.com` : beziehe ein neues Zertifikat über die apache-Autorisierung und automatische Installation des neuen Zertifikats
- `sudo certbot renew` : erneuere alle Let's Encrypt Zertifikate die in 30 Tagen oder weniger auslaufen (nicht vergessen alle Server, die diese nutzen, neu zu starten)
- `sudo certbot --webroot -w pfad/zu/webroot -d subdomain.example.com --dry-run` : simuliere die Zertifikatserneuerung, ohne diese zu speichern
- `sudo certbot --webroot -w pfad/zu/webroot -d subdomain.example.com --test-cert` : beziehe ein Test-Zertifikat

## Links
- More information: https://eff-certbot.readthedocs.io/en/latest/using.html
