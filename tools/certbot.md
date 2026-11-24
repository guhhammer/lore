# certbot

certbot gets free HTTPS certificates from Let's Encrypt and renews them. It can edit nginx or Apache for you.

## Tasks
### Get a certificate
- `sudo certbot --nginx -d example.org -d www.example.org` : get a certificate and set up nginx
- `sudo certbot --apache -d example.org` : the same for Apache
- `sudo certbot certonly --webroot -w /var/www/example -d example.org` : get one without touching the server config
- `sudo certbot certonly --standalone -d example.org` : get one using certbot's own temporary web server
- `sudo certbot certonly --manual --preferred-challenges dns -d '*.example.org'` : a wildcard certificate by DNS
### Renew
- `sudo certbot renew --dry-run` : test renewal without changing anything
- `sudo certbot renew` : renew what is near expiry
- `sudo certbot renew --force-renewal` : renew now, even if not due
- `systemctl list-timers | grep certbot` : check the automatic renewal timer
- `sudo certbot renew --deploy-hook "systemctl reload nginx"` : reload a server after a renewal
### Look
- `sudo certbot certificates` : certificates, domains and expiry dates
- `sudo ls /etc/letsencrypt/live/` : folder of each certificate
- `echo | openssl s_client -connect example.org:443 -servername example.org 2>/dev/null | openssl x509 -noout -dates` : check what a site really serves
### Change and remove
- `sudo certbot --nginx --expand -d example.org -d NEW.example.org` : add a name to a certificate
- `sudo certbot delete --cert-name example.org` : delete a certificate (deletes it)
- `sudo certbot revoke --cert-path /etc/letsencrypt/live/example.org/cert.pem` : revoke a certificate
- `sudo certbot update_account --email you@example.org` : change the contact address
### Files and tests
- `/etc/letsencrypt/live/example.org/fullchain.pem` : the certificate plus the chain, for servers
- `/etc/letsencrypt/live/example.org/privkey.pem` : the private key, keep it secret
- `sudo certbot --staging --nginx -d example.org` : try against the test server, no rate limits

## Notes
- The domain must point to your server and port 80 must be reachable for the usual checks.
- Certificates last 90 days; the package installs a timer or cron job to renew them.
- Wildcards need a DNS check; a certbot DNS plugin can automate it.
- Use `--staging` while testing: the real servers limit repeated requests.
- Install it as the official page in Links describes (snap or your package manager).

## See also
- nginx
- openssl
- systemd
- cron

## Links
- Instructions: https://certbot.eff.org/instructions
- Docs: https://eff-certbot.readthedocs.io/en/stable/using.html

## Categories
security web
