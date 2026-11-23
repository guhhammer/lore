# pacman

Gestore pacchetti di Arch Linux.

## Usage
- `sudo pacman -Syu` : sincronizza e aggiorna tutti i pacchetti
- `sudo pacman -S pacchetto` : installa un nuovo pacchetto
- `sudo pacman -Rs pacchetto` : rimuovi un pacchetto e le sue dipendenze
- `pacman -F "nome_file"` : cerca nel database un pacchetto contenente uno specifico file
- `pacman -Q` : mostra i pacchetti installati e le versioni
- `pacman -Qe` : mostra solo i pacchetti esplicitamente installati e le versioni
- `pacman -Qtdq` : mostra i pacchetti orfani (installati come dipendenze ma attualmente non richiesti da nessun altro pacchetto)
- `sudo pacman -Scc` : svuota l'intera cache di `pacman`

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
