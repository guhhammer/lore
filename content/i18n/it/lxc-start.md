# lxc-start

Avvia un container.

## Usage
- `systemctl start lxc-net` : avvia il servizio lxc
- `sudo lxc-start nome_container` : avvia un container
- `sudo lxc-start nome_container -F` : avvia un container in primo piano
- `sudo lxc-stop nome_container` : esce da un container in primo piano (esegui questo in un terminale separato)
- `sudo lxc-start nome_container -l DEBUG -o percorso/al/logfile` : scrivi log di debug in un file
- `lxc-start -?` : mostra l'aiuto

## Links
- More information: https://linuxcontainers.org/lxc/getting-started/
