# caddy

An enterprise-ready open source web server with automatic HTTPS, written in Go.

## Usage
- `caddy run` : start Caddy in the foreground
- `caddy run --config path/to/Caddyfile` : start Caddy with the specified Caddyfile
- `caddy start` : start Caddy in the background
- `caddy stop` : stop a background Caddy process
- `caddy file-server --listen :8000 --browse` : run a simple file server on the specified port with a browsable interface
- `caddy reverse-proxy --from :80 --to localhost:8000` : run a reverse proxy server

## Categories
network, files

## Links
- More information: https://caddyserver.com/docs/command-line
- tldr page: https://tldr.inbrowser.app/pages/common/caddy
