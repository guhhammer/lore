# cloudflared

Create a persistent connection to the Cloudflare network.

## Usage
- `cloudflared tunnel login` : authenticate and associate the connection to a domain in the Cloudflare account
- `cloudflared tunnel create name` : create a tunnel with a specific name
- `cloudflared tunnel list` : list all tunnels in the account
- `cloudflared tunnel route dns name|uuid hostname` : create a DNS CNAME record pointing to a tunnel
- `cloudflared tunnel --loglevel panic|fatal|error|warn|info|debug --logfile path/to/file run name` : save logs to a file
- `cloudflared tunnel run name` : run a named tunnel (reads configuration from `config.yml`)
- `cloudflared tunnel --url http://localhost:port` : start a temporary tunnel to expose a local service (no account required)
- `cloudflared service install` : install cloudflared as a system service

## Categories
network, users, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/cloudflared
