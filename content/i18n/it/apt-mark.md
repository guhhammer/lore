# apt-mark

Servizio per cambiare lo stato di un pacchetto installato.

## Usage
- `sudo apt-mark auto pacchetto` : contrassegna un pacchetto come installato automaticamente
- `sudo apt-mark hold pacchetto` : mantiene un pacchetto alla sua versione attuale e ne previene l'aggiornamento
- `sudo apt-mark unhold pacchetto` : consente ad un pacchetto di essere nuovamente aggiornato
- `apt-mark showmanual` : mostra i pacchetti installati manualmente
- `apt-mark showhold` : visualizza i pacchetti mantenuti alla versione attuale che non sono stati aggiornati

## Links
- More information: https://manned.org/apt-mark
