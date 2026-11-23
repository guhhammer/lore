# aptitude

Utilità di gestione pacchetti per Debian e Ubuntu.

## Usage
- `sudo aptitude update` : sincronizza l'elenco dei pacchetti e delle versioni disponibili. Va eseguito prima di altri comandi `aptitude`
- `sudo aptitude install pacchetto` : installa un nuovo pacchetto e le sue dipendenze
- `aptitude search pacchetto` : cerca un pacchetto
- `aptitude search '?installed(pacchetto)'` : cerca un pacchetto installato (`?installed` è un termine di ricerca di `aptitude`)
- `sudo aptitude remove pacchetto` : rimuove un pacchetto e tutti i pacchetti che dipendono da esso
- `sudo aptitude upgrade` : aggiorna i pacchetti installati alle versioni più recenti disponibili
- `sudo aptitude full-upgrade` : aggiorna i pacchetti installati (come `aptitude upgrade`) inclusa la rimozione di pacchetti obsoleti e l'installazione di pacchetti aggiuntivi per soddisfare nuove dipendenze
- `sudo aptitude hold '?installed(pacchetto)'` : mettere in pausa un pacchetto installato per impedirne l'aggiornamento automatico

## Links
- More information: https://manned.org/aptitude
