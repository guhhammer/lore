# lxc

Gestisce i container Linux usando l'API REST lxd. Qualsiasi nome di container o pattern può essere preceduto dal nome di un server remoto.

## Usage
- `lxc list stringa_di_ricerca` : elenca i container locali che corrispondono a una stringa. Ometti la stringa per elencare tutti i container locali
- `lxc image list [remote:]stringa_di_ricerca` : elenca le immagini che corrispondono a una stringa. Ometti la stringa per elencare tutte le immagini
- `lxc init [remote:]immagine container` : crea un nuovo container da un'immagine
- `lxc start [remote:]container` : avvia un container
- `lxc stop [remote:]container` : ferma un container
- `lxc info [remote:]container` : mostra informazioni dettagliate su un container
- `lxc snapshot [remote:]container snapshot` : crea uno snapshot di un container
- `lxc exec [remote:]container comando` : esegue un comando specifico all'interno di un container

## Links
- More information: https://manned.org/lxc
