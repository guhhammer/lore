# apt

Servizio di gestione dei pacchetti per distribuzioni basate su Debian. Rimpiazzo raccomandato di `apt-get` quando usato interattivamente su Ubuntu 16.04 e versioni successive.

## Usage
- `sudo apt update` : aggiorna la lista dei pacchetti e delle loro versioni disponibili (è consigliato eseguire questo comando prima di altri comandi `apt`)
- `apt search pacchetto` : cerca per un dato pacchetto
- `apt show pacchetto` : mostra le informazioni su un pacchetto
- `sudo apt install pacchetto` : installa un pacchetto, o lo aggiorna all'ultima versione disponibile
- `sudo apt remove pacchetto` : rimuove un pacchetto (usando `purge` rimuove anche i suoi file di configurazione)
- `sudo apt upgrade` : aggiorna tutti i pacchetti installati alla versione disponibile più recente
- `apt list` : elenca tutti i pacchetti
- `apt list -i` : elenca i pacchetti installati

## Links
- More information: https://manned.org/apt.8
