# apt-get

Servizio di gestione dei pacchetti per Debian e Ubuntu. Cerca i pacchetti usando `apt-cache`.

## Usage
- `sudo apt-get update` : aggiorna la lista dei pacchetti e delle loro versioni disponibili (è consigliato eseguire questo comando prima di altri comandi `apt-get`)
- `sudo apt-get install pacchetto` : installa un pacchetto, o lo aggiorna all'ultima versione disponibile
- `sudo apt-get remove pacchetto` : rimuove un pacchetto
- `sudo apt-get purge pacchetto` : rimuove un pacchetto ed i suoi file di configurazione
- `sudo apt-get upgrade` : aggiorna tutti i pacchetti installati alla versione disponibile più recente
- `sudo apt-get autoclean` : pulisce gli archivi locali - rimuovendo i file (`.deb`) da scaricamenti interrotti che non possono più essere scaricati
- `sudo apt-get autoremove` : rimuove tutti i pacchetti che non sono più necessari
- `sudo apt-get dist-upgrade` : aggiorna tutti i pacchetti installati (come `upgrade`), rimuovendo i pacchetti obsoleti ed installando ulteriori pacchetti per soddisfare le nuove dipendenze

## Links
- More information: https://manned.org/apt-get.8
