# dpkg

Gestore di pacchetti Debian. Alcuni sottocomandi, come `deb`, hanno la propria documentazione d'uso.

## Usage
- `sudo dpkg -i percorso/del/file.deb` : installa un pacchetto
- `sudo dpkg -r pacchetto` : rimuove un pacchetto
- `dpkg -l espressione` : elenca i pacchetti installati
- `dpkg -L pacchetto` : elenca i contenuti di un pacchetto
- `dpkg -c percorso/del/file.deb` : elenca i contenuti di un file pacchetto locale
- `dpkg -S percorso/del/file` : trova a quale pacchetto appartiene un file
- `sudo dpkg -P pacchetto` : rimuove completamente un pacchetto installato o già rimosso, inclusa la configurazione

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
