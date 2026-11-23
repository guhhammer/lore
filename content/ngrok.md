# ngrok

Reverse proxy that creates a secure tunnel from a public endpoint to a locally running web service.

## Usage
- `ngrok http 80` : expose a local HTTP service on a given port
- `ngrok http example.com:80` : expose a local HTTP service on a specific host
- `ngrok http https://localhost` : expose a local HTTPS server
- `ngrok tcp 22` : expose TCP traffic on a given port
- `ngrok tls -hostname=example.com 443` : expose TLS traffic for a specific host and port

## Categories
network, security, system

## Links
- More information: https://ngrok.com/docs/agent/cli
- tldr page: https://tldr.inbrowser.app/pages/common/ngrok
