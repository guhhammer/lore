# dnsmasq

Server DNS, DHCP, TFTP e PXE leggero.

## Usage
- `dnsmasq` : avvia dnsmasq con configurazione predefinita
- `dnsmasq --no-daemon` : esegui dnsmasq in primo piano (per debug)
- `dnsmasq --conf-file=percorso/del/config.conf` : specifica un file di configurazione personalizzato
- `dnsmasq --log-queries --log-facility=-` : abilita logging verboso
- `dnsmasq --dhcp-range=192.168.0.50,192.168.0.150,12h` : imposta un intervallo DHCP e durata lease
- `dnsmasq --version` : mostra la versione

## Links
- More information: https://manned.org/dnsmasq
