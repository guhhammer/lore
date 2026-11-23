# tailscale funnel

Share a local server on the internet using Tailscale.

## Usage
- `tailscale funnel /path/to/file_or_directory` : expose a local file or directory in the foreground
- `tailscale funnel 3000` : expose an HTTP server running at 127.0.0.1:3000 in the foreground
- `tailscale funnel --bg 3000` : expose an HTTP server running at 127.0.0.1:3000 in the background
- `tailscale funnel https+insecure://localhost:8443` : expose an HTTPS server with invalid or self-signed certificates at <https://localhost:8443>
- `tailscale funnel status` : display the current configuration
- `tailscale funnel status --json` : display the current configuration in JSON format
- `tailscale funnel reset` : reset the current configuration

## Categories
network, files

## Links
- More information: https://tailscale.com/kb/1311/tailscale-funnel
- tldr page: https://tldr.inbrowser.app/pages/common/tailscale-funnel
