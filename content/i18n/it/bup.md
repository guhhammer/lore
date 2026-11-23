# bup

Sistema di backup basato sul formato dei packfile Git, fornendo salvataggi incrementali veloci e deduplicazione globale.

## Usage
- `bup -d percorso/della/repository init` : inizializza una repository di backup nella directory locale specificata
- `bup -d percorso/della/repository index percorso/della/directory` : prepara una certa directory prima di fare un backup
- `bup -d percorso/della/repository save -n nome_backup percorso/della/directory` : esegui il backup di una directory in una repository locale
- `bup -d percorso/della/repository ls` : elenca i di backup attualmente nella repository
- `bup -d percorso/della/repository restore -C percorso/della/destinazione nome_backup` : ripristina uno specifico backup in una determinata directory locale

## Links
- More information: https://manned.org/bup
