# brew services

Gestisci i servizi in background con `launchctl` su macOS o `systemctl` su Linux.

## Usage
- `brew services` : elenca tutti i servizi gestiti per l'utente corrente
- `brew services info --all` : elenca ulteriori informazioni su tutti i servizi gestiti
- `brew services start formula` : avvia immediatamente un servizio e registralo per l'avvio al login (o all'avvio)
- `brew services stop formula` : ferma immediatamente il servizio e rimuovilo dall'avvio al login (o all'avvio)
- `brew services restart formula` : ferma (se necessario) e avvia immediatamente il servizio, quindi registralo per l'avvio al login (o all'avvio)
- `brew services cleanup` : rimuovi tutti i servizi non più usati

## Links
- More information: https://docs.brew.sh/Manpage#services-subcommand
