# cockpit-ws

Comunica tra l'applicazione browser e vari strumenti di configurazione e servizi come `cockpit-bridge`.

## Usage
- `cockpit-ws --local-ssh` : avvia con autenticazione tramite SSH su `127.0.0.1` con porta `22` abilitata
- `cockpit-ws --port porta` : avvia un server HTTP su una porta specifica
- `cockpit-ws --address ip_address` : avvia e si associa a un indirizzo IP specifico (predefinito `0.0.0.0`)
- `cockpit-ws --no-tls` : avvia senza TLS
- `cockpit-ws --help` : mostra l'aiuto

## Links
- More information: https://cockpit-project.org/guide/latest/cockpit-ws.8.html
