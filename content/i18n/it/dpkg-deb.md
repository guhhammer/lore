# dpkg-deb

Impacchetta, spacchetta e fornisce informazioni su archivi Debian.

## Usage
- `dpkg-deb -I percorso/del/file.deb` : mostra le informazioni riguardo ad un pacchetto
- `dpkg-deb -W percorso/del/file.deb` : mostra il nome e la versione del pacchetto in una singola riga
- `dpkg-deb -c percorso/del/file.deb` : elenca i contenuti del pacchetto
- `dpkg-deb -x percorso/del/file.deb percorso/della/directory` : estrae i contenuti del pacchetto in una directory
- `dpkg-deb -b percorso/della/directory` : crea una pacchetto a partire da una directory specificata

## Links
- More information: https://manned.org/dpkg-deb
