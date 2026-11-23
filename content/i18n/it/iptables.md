# iptables

Programma che permette di configurare tabelle, catene e regole fornite dal firewall del kernel Linux.

## Usage
- `sudo iptables -vnL --line-numbers` : visualizza catene, regole e contatori di pacchetti/byte per la tabella dei filtri
- `sudo iptables -P catena regola` : imposta regola ad una catena
- `sudo iptables -A catena -s ip_address -j regola` : appendi regola ad una catena di policy per IP
- `sudo iptables -A catena -s ip_address -p protocollo --dport porta -j regola` : appendi regola ad una catena di policy per IP considerando protocollo e porta
- `sudo iptables -D catena numero_di_linea_della_regola` : cancella regola da una catena
- `sudo iptables-save -t nome tabella >percorso/del/file_iptables` : salva la configurazione di ip tables di una specifica tabella in un file
- `sudo iptables-restore <percorso/del/file_iptables` : ripristina la configurazione di iptables da un file

## Links
- More information: https://manned.org/iptables
