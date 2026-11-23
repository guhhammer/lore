# certbot

De Let's Encrypt Agent om automatisch TLS-certificaten te verkrijgen en te vernieuwen. Opvolger van `letsencrypt`.

## Usage
- `sudo certbot certonly --webroot -w pad/naar/webroot -d subdomein.example.com` : verkrijg een nieuw certificaat via webroot-autorisatie, maar installeer het certificaat niet automatisch
- `sudo certbot --nginx -d subdomein.example.com` : verkrijg een nieuw certificaat via `nginx`-autorisatie, installeer het nieuwe certificaat automatisch
- `sudo certbot --apache -d subdomein.example.com` : verkrijg een nieuw certificaat via apache-autorisatie, installeer het nieuwe certificaat automatisch
- `sudo certbot renew` : vernieuw alle Let's Encrypt-certificaten die binnen 30 dagen verlopen (vergeet achteraf niet alle servers te herstarten die deze gebruiken)
- `sudo certbot --webroot -w pad/naar/webroot -d subdomein.example.com --dry-run` : simuleer het verkrijgen van een nieuw certificaat, zonder daadwerkelijk nieuwe certificaten op de schijf op te slaan
- `sudo certbot --webroot -w pad/naar/webroot -d subdomein.example.com --test-cert` : verkrijg in plaats daarvan een onvertrouwd testcertificaat

## Links
- More information: https://eff-certbot.readthedocs.io/en/latest/using.html
