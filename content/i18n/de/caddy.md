# caddy

Ein unternehmenstauglicher Open-Source-Webserver mit automatischem HTTPS, geschrieben in Go.

## Usage
- `caddy run` : starte Caddy im Vordergrund
- `caddy run --config pfad/zu/Caddyfile` : starte Caddy mit dem angegebenen Caddyfile
- `caddy start` : starte Caddy im Hintergrund
- `caddy stop` : stoppe einen im Hintergrund laufenden Caddy-Prozess
- `caddy file-server --listen :8000 --browse` : führe einen einfachen Dateiserver auf dem angegebenen Port mit einer durchsuchbaren Oberfläche aus
- `caddy reverse-proxy --from :80 --to localhost:8000` : führe einen Reverse-Proxy-Server aus

## Links
- More information: https://caddyserver.com/docs/command-line
