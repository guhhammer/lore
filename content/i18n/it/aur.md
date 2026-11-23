# aur

Compila pacchetti dall’AUR e gestisci repository locali. Nota: è necessario definire un repository locale in `/etc/pacman.conf` e installare `vifm` per avere tutte le funzionalità.

## Usage
- `repo-add percorso/del/database.db.tar.gz` : inizializza il repository che corrisponde al percorso in `/etc/pacman.conf`
- `aur search parola_chiave` : cerca un pacchetto nel database AUR
- `aur sync pacchetto1 pacchetto2 ...` : scarica uno o più pacchetti e le loro dipendenze dall’AUR, compilali e aggiungili a un repository locale
- `aur repo -l` : elenca i pacchetti disponibili nel tuo repository locale
- `aur sync -u` : aggiorna i pacchetti del repository locale
- `aur sync -C pacchetto` : pulisci i file di compilazione dopo l’installazione
- `aur sync --noview -n pacchetto` : installa un pacchetto senza mostrare le modifiche in Vim e senza confermare l’installazione delle dipendenze
- `repo-remove percorso/del/database.db.tar.gz pacchetto` : rimuovi un pacchetto dai metadati del repository (non rimuove il file del pacchetto stesso)

## Links
- More information: https://github.com/aurutils/aurutils
