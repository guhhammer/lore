# crond

Servizio per eseguire i comandi pianificati dai file crontab.

## Usage
- `crond` : avvia un servizio in background e controlla i comandi pianificati
- `crond -n` : avvia un servizio in primo piano e controlla i comandi pianificati
- `crond -s` : invia l'output del job dal servizio al system log
- `crond -p` : sovrascrive le limitazioni di default e accetta crontables personalizzate
- `crond -P` : eredita il percorso del file crontab dalle impostazioni dell'ambiente

## Links
- More information: https://manned.org/crond
