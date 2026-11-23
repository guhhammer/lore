# nginx

`nginx` webserver.

## Usage
- `nginx` : start de server met het standaard configuratiebestand
- `nginx -c configuratiebestand` : start de server met een aangepast configuratiebestand
- `nginx -c configuratiebestand -p pad/naar/prefix` : start de server met een prefix voor alle relatieve paden in het configuratiebestand
- `nginx -t` : test de configuratie zonder de actieve server te beïnvloeden
- `nginx -s reload` : herlaad de configuratie door een signaal te sturen zonder downtime

## Links
- More information: https://nginx.org/docs/switches.html
