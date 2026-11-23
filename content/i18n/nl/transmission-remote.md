# transmission-remote

Externe besturingshulpprogramma voor `transmission-daemon` en `transmission`.

## Usage
- `transmission-remote hostnaam -a torrent|url -w /pad/naar/download_map` : voeg een torrentbestand of magnet-link toe aan Transmission en download naar een opgegeven map
- `transmission-remote hostnaam -w /pad/naar/download_map` : verander de standaard downloadmap
- `transmission-remote hostnaam -l` : toon alle torrents
- `transmission-remote hostnaam -t "1,2" -s -t 3 -S` : start torrent 1 en 2, stop torrent 3
- `transmission-remote hostnaam -t 1 -r -t 2 -rad` : verwijder torrent 1 en 2 en verwijder ook alle lokale gegevens voor torrent 2
- `transmission-remote hostnaam -t all -S` : stop alle torrents
- `transmission-remote hostnaam -t "1-10,15-20" --move /pad/naar/nieuwe_map` : verplaats torrents 1-10 en 15-20 naar een nieuwe map (die wordt aangemaakt als deze nog niet bestaat)

## Links
- More information: https://manned.org/transmission-remote
