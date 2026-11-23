# ngrok

Reverse proxy che crea un tunnel sicuro da un endpoint pubblico a un servizio web in esecuzione in locale.

## Usage
- `ngrok http 80` : esponi un servizio HTTP locale su una determinata porta
- `ngrok http example.com:80` : esponi un servizio HTTP locale su un host specifico
- `ngrok http https://localhost` : esponi un server HTTPS locale
- `ngrok tcp 22` : esponi il traffico TCP su una determinata porta
- `ngrok tls -hostname=example.com 443` : esponi il traffico TLS per un host e una porta specifici

## Links
- More information: https://ngrok.com/docs/agent/cli
