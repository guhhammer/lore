# ngrok

Reverse-Proxy, welcher einen sicheren Tunnel von einem öffentlichen Endpunkt zu einem lokal verfügbaren Webservice erstellt.

## Usage
- `ngrok http 80` : veröffentliche einen lokalen HTTP-Service auf dem angegebenen Port
- `ngrok http example.com:80` : veröffentliche einen lokalen HTTP-Service auf einem bestimmten Host
- `ngrok http https://localhost` : veröffentliche einen lokalen HTTPS-Server
- `ngrok tcp 22` : veröffentliche den TCP-Traffic auf dem angegebenen Port
- `ngrok tls -hostname=example.com 443` : veröffentliche den TLS-Traffic für einen bestimmten Host und Port

## Links
- More information: https://ngrok.com/docs/agent/cli
