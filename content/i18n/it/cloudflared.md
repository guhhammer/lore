# cloudflared

Crea una connessione persistente alla rete Cloudflare.

## Usage
- `cloudflared tunnel login` : autentica e associa la connessione a un dominio nell'account Cloudflare
- `cloudflared tunnel create nome` : crea un tunnel con un nome specifico
- `cloudflared tunnel --hostname hostname localhost:numero_porta` : stabilisce un tunnel verso un host in Cloudflare dal server locale
- `cloudflared tunnel --hostname hostname localhost:numero_porta --no-tls-verify` : stabilisce un tunnel verso un host in Cloudflare dal server locale, senza verificare il certificato del server locale
- `cloudflared tunnel --hostname hostname http://localhost:numero_porta --loglevel panic|fatal|error|warn|info|debug --logfile percorso/del/file` : salva i log in un file
- `cloudflared service install` : installa cloudflared come servizio di sistema
