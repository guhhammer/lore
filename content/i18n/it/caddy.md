# caddy

Server web open source enterprise-ready con HTTPS automatico, scritto in Go.

## Usage
- `caddy run` : avvia Caddy in primo piano
- `caddy run --config percorso/del/Caddyfile` : avvia Caddy con il Caddyfile specificato
- `caddy start` : avvia Caddy in background
- `caddy stop` : ferma un processo Caddy in background
- `caddy file-server --listen :8000 --browse` : esegue un semplice file server sulla porta specificata con interfaccia navigabile
- `caddy reverse-proxy --from :80 --to localhost:8000` : esegue un server reverse proxy

## Links
- More information: https://caddyserver.com/docs/command-line
