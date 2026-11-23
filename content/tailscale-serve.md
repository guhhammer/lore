# tailscale serve

Share a local server, file, or directory with devices on your Tailscale network.

## Usage
- `tailscale serve 3000` : expose an HTTP server running at 127.0.0.1:3000 in the foreground
- `tailscale serve --bg 3000` : expose an HTTP server running at 127.0.0.1:3000 in the background
- `tailscale serve /path/to/file_or_directory` : expose a local file or directory
- `tailscale serve --https 8443 localhost:3000` : expose a local server on a specific HTTPS port
- `tailscale serve --https 443 off` : stop serving on a specific HTTPS port
- `tailscale serve status` : display the current serve configuration
- `tailscale serve status --json` : display the current serve configuration in JSON format
- `tailscale serve reset` : remove all serve configuration

## See also
- tailscale funnel

## Categories
network, files, hardware

## Links
- More information: https://tailscale.com/docs/reference/tailscale-cli/serve
- tldr page: https://tldr.inbrowser.app/pages/common/tailscale-serve
