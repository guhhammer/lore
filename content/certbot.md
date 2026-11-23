# certbot

The Let's Encrypt Agent for automatically obtaining and renewing TLS certificates. Successor to `letsencrypt`.

## Usage
- `sudo certbot certonly --webroot -w path/to/webroot -d subdomain.example.com` : obtain a new certificate via webroot authorization, but do not install it automatically
- `sudo certbot --nginx -d subdomain.example.com` : obtain a new certificate via `nginx` authorization, installing the new certificate automatically
- `sudo certbot --apache -d subdomain.example.com` : obtain a new certificate via apache authorization, installing the new certificate automatically
- `sudo certbot renew` : renew all Let's Encrypt certificates that expire in 30 days or less (don't forget to restart any servers that use them afterwards)
- `sudo certbot --webroot -w path/to/webroot -d subdomain.example.com --dry-run` : simulate the obtaining of a new certificate without actually saving any new certificates to disk
- `sudo certbot --webroot -w path/to/webroot -d subdomain.example.com --test-cert` : obtain an untrusted test certificate instead

## Categories
security, users, files

## Links
- More information: https://eff-certbot.readthedocs.io/en/latest/using.html
- tldr page: https://tldr.inbrowser.app/pages/linux/certbot
