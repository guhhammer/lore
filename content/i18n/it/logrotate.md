# logrotate

Ruota, comprime e invia via email i log di sistema.

## Usage
- `logrotate percorso/del/logrotate.conf --force` : attiva un'esecuzione manualmente
- `logrotate percorso/del/logrotate.conf --mail /usr/bin/mail_command` : esegui utilizzando un comando specifico per inviare rapporti via email
- `logrotate percorso/del/logrotate.conf --state /dev/null` : esegui senza utilizzare un file di stato (blocco)
- `logrotate percorso/del/logrotate.conf --skip-state-lock` : esegui e salta il controllo del file di stato (blocco)
- `logrotate percorso/del/logrotate.conf --log percorso/del/file_log` : dì a `logrotate` di registrare l'output dettagliato nel file di log

## Links
- More information: https://manned.org/logrotate
