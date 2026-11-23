# dpkg-query

Mostra informazioni sui pacchetti installati.

## Usage
- `dpkg-query -l` : elenca tutti i pacchetti installati
- `dpkg-query -l 'libc6*'` : elenca i pacchetti installati che corrispondono a un pattern
- `dpkg-query -L libc6` : elenca tutti i file installati da un pacchetto
- `dpkg-query -s libc6` : mostra informazioni su un pacchetto
- `dpkg-query -S /etc/ld.so.conf.d` : cerca i pacchetti che possiedono file corrispondenti a un pattern

## Links
- More information: https://manned.org/dpkg-query
